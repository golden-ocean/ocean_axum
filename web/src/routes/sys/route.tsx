import DefaultLayout from "@/components/layout";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/sys")({
  component: DefaultLayout,
});
