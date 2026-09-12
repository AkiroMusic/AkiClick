/**
 * Windows VK code <-> display name mapping.
 * Keep in sync with `vk_to_code` in src-tauri/src/hotkeys.rs.
 */
export const VK_CODES: Record<number, string> = {
  112: 'F1', 113: 'F2', 114: 'F3', 115: 'F4',
  116: 'F5', 117: 'F6', 118: 'F7', 119: 'F8',
  120: 'F9', 121: 'F10', 122: 'F11', 123: 'F12',
  48: '0', 49: '1', 50: '2', 51: '3', 52: '4',
  53: '5', 54: '6', 55: '7', 56: '8', 57: '9',
  65: 'A', 66: 'B', 67: 'C', 68: 'D', 69: 'E',
  70: 'F', 71: 'G', 72: 'H', 73: 'I', 74: 'J',
  75: 'K', 76: 'L', 77: 'M', 78: 'N', 79: 'O',
  80: 'P', 81: 'Q', 82: 'R', 83: 'S', 84: 'T',
  85: 'U', 86: 'V', 87: 'W', 88: 'X', 89: 'Y', 90: 'Z',
  96: 'Num0', 97: 'Num1', 98: 'Num2', 99: 'Num3', 100: 'Num4',
  101: 'Num5', 102: 'Num6', 103: 'Num7', 104: 'Num8', 105: 'Num9',
}

export const NAME_TO_VK: Record<string, number> = Object.fromEntries(
  Object.entries(VK_CODES).map(([vk, name]) => [name, parseInt(vk)])
)

export const HOTKEY_OPTIONS = Object.entries(VK_CODES).map(([vk, name]) => ({
  value: parseInt(vk),
  label: name,
}))

export function vkToName(vk: number): string {
  return VK_CODES[vk] ?? String(vk)
}

export function nameToVk(name: string): number {
  return NAME_TO_VK[name] ?? 120
}
