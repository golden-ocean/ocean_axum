import TanstackDevtoolsProvider from "@/providers/tanstack-devtools";
import { createRootRoute, Outlet } from "@tanstack/react-router";

export const Route = createRootRoute({
  component: () => (
    <>
      <main>
        <Outlet />
      </main>
      <TanstackDevtoolsProvider />
    </>
  ),
});
