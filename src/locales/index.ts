import zhCN from './zh-CN'
import enUS from './en-US'

export const messages = {
  'zh-CN': zhCN,
  'en-US': enUS,
}

export type LocaleType = keyof typeof messages

