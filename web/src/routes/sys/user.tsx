import UserPage from "@/features/iam/user";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/sys/user")({
  component: UserPage,
});
