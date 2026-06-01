import TanstackQueryProvider from "./tanstack-query";
import TanstackRouterProvider from "./tanstack-router";
import ThemeProvider from "./theme";
import ShadcnProvider from "./shadcn";

export const RootProvider = () => {
  return (
    <ThemeProvider defaultTheme="system">
      <TanstackQueryProvider>
        <ShadcnProvider>
          <TanstackRouterProvider />
        </ShadcnProvider>
      </TanstackQueryProvider>
    </ThemeProvider>
  );
};
