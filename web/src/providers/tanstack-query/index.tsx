import { QueryClientProvider } from "@tanstack/react-query";
import { TanstackQueryContext } from "./tanstack-query-context";

export default function TanstackQueryProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const queryClient = TanstackQueryContext();
  return (
    <QueryClientProvider client={queryClient.queryClient}>
      {children}
    </QueryClientProvider>
  );
}
