import { ITerminalOptions } from "@xterm/xterm";

export const xtermjsTheme = {
  foreground: "#F8F8F8",
  background: "#1a1a1a",
  selectionBackground: "#5DA5D533",
  selectionInactiveBackground: "#555555AA",
  black: "#1E1E1D",
  brightBlack: "#262625",
  red: "#CE5C5C",
  brightRed: "#FF7272",
  green: "#5BCC5B",
  brightGreen: "#72FF72",
  yellow: "#CCCC5B",
  brightYellow: "#FFFF72",
  blue: "#5D5DD3",
  brightBlue: "#7279FF",
  magenta: "#BC5ED1",
  brightMagenta: "#E572FF",
  cyan: "#5DA5D5",
  brightCyan: "#72F0FF",
  white: "#F8F8F8",
  brightWhite: "#FFFFFF",
};
const isWindows =
  ["Windows", "Win16", "Win32", "WinCE"].indexOf(navigator.platform) >= 0;
export const config: ITerminalOptions = {
  allowProposedApi: true,
  windowsPty: isWindows
    ? {
        backend: "conpty",
        buildNumber: 22621,
      }
    : undefined,
  fontFamily: '"Fira Code", monospace, "Powerline Extra Symbols"',
  theme: xtermjsTheme,
  cursorBlink: true,
  cursorStyle: "bar",
  lineHeight: 1.2,
  letterSpacing: 0,
  allowTransparency: false,
};
const lineBreakRegex = /\r\n|\r|\n/;
const defaultPrompt = "\r\n$ ";
export const splitShellDataToLines = (
  result?: string | number,
  shouldRemoveFirstAndLastLine: boolean = true
): { lines: Array<string>; prompt: string } => {
  if (result === null || result === undefined)
    return { lines: [], prompt: defaultPrompt };
  const resultString = result.toString();

  // 兼容多平台换行符：\n, \r, \r\n
  const allLines = resultString.split(lineBreakRegex);
  const prompt = allLines[allLines.length - 1];

  /**
   * shellData
   * 第一行是上次的命令，最后一行是提示符
   * 第一行替换为空字符串，去掉最后一行
   */
  let lines = allLines;
  console.log("🚀 ~ splitShellDataToLines ~ allLines:", allLines)
  if (shouldRemoveFirstAndLastLine && allLines.length > 0) {
    lines = allLines.slice(0, -1); // 去掉最后一行（提示符）
    if (lines.length > 0) {
      lines[0] = ""; // 第一行（上次的命令）替换为空字符串
    }
  }

  return { lines, prompt };
};
/**
 * 更新提示符
 * 返回的数据的最后一行是提示符，拿到后，返回
 * @param result
 */
export const getPrompt = (result?: string | number) => {
  if (result === null || result === undefined) return defaultPrompt;
  const { prompt } = splitShellDataToLines(result, false);

  return prompt;
};
