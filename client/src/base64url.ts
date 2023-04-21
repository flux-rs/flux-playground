import * as utf8 from "./utf8";

// private property
const KEY_STR = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_=";

// public method for encoding
export const encode_text = (input: string): string => {
  const bytes = utf8.encode(input);
  return encode(bytes);
};

export const decode_text = (input: string): string => {
  const bytes = decode(input);
  return utf8.decode(bytes);
};

export const encode = (input: number[]): string => {
  let output = "";
  let i = 0;

  while (i < input.length) {
    const chr1 = input[i++];
    const chr2 = input[i++];
    const chr3 = input[i++];

    const enc1 = chr1 >> 2;
    const enc2 = ((chr1 & 3) << 4) | (chr2 >> 4);
    let enc3 = ((chr2 & 15) << 2) | (chr3 >> 6);
    let enc4 = chr3 & 63;

    if (isNaN(chr2)) {
      enc3 = 64;
      enc4 = 64;
    } else if (isNaN(chr3)) {
      enc4 = 64;
    }

    output =
      output +
      KEY_STR.charAt(enc1) +
      KEY_STR.charAt(enc2) +
      KEY_STR.charAt(enc3) +
      KEY_STR.charAt(enc4);
  }
  return output;
};

// public method for decoding
export const decode = (input: string): number[] => {
  let bytes = [];
  let i = 0;

  while (i < input.length) {
    const enc1 = KEY_STR.indexOf(input.charAt(i++));
    const enc2 = KEY_STR.indexOf(input.charAt(i++));
    const enc3 = KEY_STR.indexOf(input.charAt(i++));
    const enc4 = KEY_STR.indexOf(input.charAt(i++));

    const chr1 = (enc1 << 2) | (enc2 >> 4);
    const chr2 = ((enc2 & 15) << 4) | (enc3 >> 2);
    const chr3 = ((enc3 & 3) << 6) | enc4;

    bytes.push(chr1);

    if (enc3 != 64) {
      bytes.push(chr2);
    }
    if (enc4 != 64) {
      bytes.push(chr3);
    }
  }
  return bytes;
};
