import { ref } from 'vue'

type ToastType = 'success' | 'error' | 'warning' | 'info'

const visible = ref(false)
const message = ref('')
const type = ref<ToastType>('info')
let timer: ReturnType<typeof setTimeout> | null = null

export function useToast() {
  const show = (msg: string, toastType: ToastType = 'info', duration = 3000) => {
    if (timer) {
      clearTimeout(timer)
    }
    
    message.value = msg
    type.value = toastType
    visible.value = true
    
    timer = setTimeout(() => {
      visible.value = false
    }, duration)
  }

  const success = (msg: string, duration?: number) => {
    show(msg, 'success', duration)
  }

  const error = (msg: string, duration?: number) => {
    show(msg, 'error', duration)
  }

  const warning = (msg: string, duration?: number) => {
    show(msg, 'warning', duration)
  }

  const info = (msg: string, duration?: number) => {
    show(msg, 'info', duration)
  }

  const hide = () => {
    visible.value = false
    if (timer) {
      clearTimeout(timer)
      timer = null
    }
  }

  return {
    visible,
    message,
    type,
    show,
    success,
    error,
    warning,
    info,
    hide,
  }
}

