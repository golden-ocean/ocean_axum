import RolePage from "@/features/iam/role";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/sys/role")({
  component: RolePage,
});
