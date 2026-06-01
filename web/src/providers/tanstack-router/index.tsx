import { createRouter, RouterProvider } from "@tanstack/react-router";
import { routeTree } from "@/routeTree.gen";
import { TanstackQueryContext } from "../tanstack-query/tanstack-query-context";

const router = createRouter({
  routeTree,
  defaultPreload: "intent",
  scrollRestoration: true,
  context: TanstackQueryContext(),
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
export default function TanstackRouterProvider() {
  return (
    <>
      <RouterProvider router={router} />
    </>
  );
}
