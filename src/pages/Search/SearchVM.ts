import { CopyToClipboardAction, DoNothingAction, ExtensionAction, OpenAppAction, OpenInBrowserAction, SimpleKlResult } from "@/data";

export function isOpenAppAction(obj: any): obj is OpenAppAction{
    return 'desktop_path' in obj
}

export function isOpenInBrowserAction(obj: any): obj is OpenInBrowserAction{
    return 'url' in obj
}

export function isCopyToClipboardAction(obj: any): obj is CopyToClipboardAction{
    return 'text' in obj
}

export function isExtensionAction(obj: any): obj is ExtensionAction{
    return 'action' in obj && 'args' in obj
}

export function isDoNothingAction(obj: any): obj is DoNothingAction{
    return '' in obj
}

export function isTextResult(result: SimpleKlResult): boolean {
  return result.icon === undefined 
  && result.icon_color === undefined 
  && result.title === undefined 
  && result.text !== undefined 
  && result.description === undefined
}

export function isIconWithTextResult(result: SimpleKlResult): boolean {
  return result.icon !== undefined 
  && result.title === undefined 
  && result.text !== undefined 
  && result.description === undefined
}

export function isTitleAndDescriptionResult(result: SimpleKlResult): boolean {
  return result.icon === undefined 
  && result.icon_color === undefined 
  && result.title !== undefined 
  && result.text === undefined 
  && result.description !== undefined
}

export function isIconWithTitleAndDescriptionResult(result: SimpleKlResult): boolean {
  return result.icon !== undefined
  && result.title !== undefined 
  && result.text === undefined 
  && result.description !== undefined
}