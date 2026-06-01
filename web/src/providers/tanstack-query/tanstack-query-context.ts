import { QueryClient } from "@tanstack/react-query";

export function TanstackQueryContext() {
  const queryClient = new QueryClient();

  return {
    queryClient,
  };
}
