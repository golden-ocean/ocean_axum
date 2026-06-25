use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN; // 64字节物理切面 (512位熵)
const PBKDF2_ITERATIONS: u32 = 100_000; // 10万次高频迭代（符合现代安全合规）

/// ### 🛡️ 基于 ring 的高性能分立式密码加盐散列器
/// 全自动利用底层操作系统硬件真随机数（CSPRNG）派生安全盐，并计算 SHA512-PBKDF2 指纹。
///
/// #### 返回值
/// `Result<(password_hash_hex, salt_hex), Unspecified>`
pub fn hash_password(plain_password: &str) -> Result<(String, String), Unspecified> {
    let n_iter = NonZeroU32::new(PBKDF2_ITERATIONS).unwrap();
    let rng = rand::SystemRandom::new();

    // 1. 真随机派发专属安全盐
    let mut salt_bytes = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt_bytes)?;

    // 2. 调度 ring 计算出分立的原始哈希字节流
    let mut hash_bytes = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt_bytes,
        plain_password.as_bytes(),
        &mut hash_bytes,
    );

    // 3. 完美咬合双列物理数据库：转换为大写（或小写）的十六进制安全 VARCHAR 字符串
    // 这里使用 data_encoding 或 hex 库均可，此处以人类可读的 hex 为准
    let password_hash_hex = hex::encode(hash_bytes);
    let salt_hex = hex::encode(salt_bytes);

    Ok((password_hash_hex, salt_hex))
}

/// ### 🔑 ring 原生恒定时间反向验证安检门
/// 接收登录明文、数据库分立捞出的 salt_hex 和预计的 hash_hex。
/// `ring::pbkdf2::verify` 内部是恒定时间比较，天生免除侧信道时序攻击。
pub fn verify_password(plain_password: &str, salt_hex: &str, expected_hash_hex: &str) -> bool {
    let n_iter = NonZeroU32::new(PBKDF2_ITERATIONS).unwrap();

    // 反序列化解析历史物理盐与哈希资产，格式若坏，直接斩立决
    let salt_bytes = match hex::decode(salt_hex) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    let expected_hash_bytes = match hex::decode(expected_hash_hex) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    // 临门一脚：投喂给 ring 验证算子，直接判断是否 is_ok()
    pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt_bytes,
        plain_password.as_bytes(),
        &expected_hash_bytes,
    )
    .is_ok()
}
