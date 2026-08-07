import { ref } from "vue";

export interface Toast {
  id: number;
  message: string;
  type: "error" | "success";
}

const toasts = ref<Toast[]>([]);
let nextId = 0;

export function useToast() {
  function show(message: string, type: "error" | "success" = "error") {
    const id = nextId++;
    toasts.value.push({ id, message, type });
    setTimeout(() => {
      toasts.value = toasts.value.filter((t) => t.id !== id);
    }, 3000);
  }

  return { toasts, show };
}