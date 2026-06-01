import { TooltipProvider } from "@/components/ui/tooltip";

export default function ShadcnProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <>
      <TooltipProvider>{children}</TooltipProvider>
    </>
  );
}
