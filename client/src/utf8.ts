// private method for UTF-8 encoding
export const encode = (str: string): number[] => {
  str = str.replace(/\r\n/g, "\n");
  let bytes = [];

  for (let n = 0; n < str.length; n++) {
    const c = str.charCodeAt(n);

    if (c < 128) {
      bytes.push(c);
    } else if (c > 127 && c < 2048) {
      bytes.push((c >> 6) | 192);
      bytes.push((c & 63) | 128);
    } else {
      bytes.push((c >> 12) | 224);
      bytes.push(((c >> 6) & 63) | 128);
      bytes.push((c & 63) | 128);
    }
  }
  return bytes;
};

// private method for UTF-8 decoding
export const decode = (bytes: number[]): string => {
  let str = "";
  let i = 0;

  while (i < bytes.length) {
    const c = bytes[i];

    if (c < 128) {
      str += String.fromCharCode(c);
      i++;
    } else if (c > 191 && c < 224) {
      const c2 = bytes[i + 1];
      str += String.fromCharCode(((c & 31) << 6) | (c2 & 63));
      i += 2;
    } else {
      const c2 = bytes[i + 1];
      const c3 = bytes[i + 2];
      str += String.fromCharCode(((c & 15) << 12) | ((c2 & 63) << 6) | (c3 & 63));
      i += 3;
    }
  }
  return str;
};
