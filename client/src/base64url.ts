// private property
const KEY_STR = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_=";

// public method for encoding
export const encode = (input: string): string => {
  let output = "";
  let i = 0;

  input = utf8_encode(input);

  while (i < input.length) {
    const chr1 = input.charCodeAt(i++);
    const chr2 = input.charCodeAt(i++);
    const chr3 = input.charCodeAt(i++);

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
export const decode = (input: string): string => {
  let output = "";
  let i = 0;

  while (i < input.length) {
    const enc1 = KEY_STR.indexOf(input.charAt(i++));
    const enc2 = KEY_STR.indexOf(input.charAt(i++));
    const enc3 = KEY_STR.indexOf(input.charAt(i++));
    const enc4 = KEY_STR.indexOf(input.charAt(i++));

    const chr1 = (enc1 << 2) | (enc2 >> 4);
    const chr2 = ((enc2 & 15) << 4) | (enc3 >> 2);
    const chr3 = ((enc3 & 3) << 6) | enc4;

    output = output + String.fromCharCode(chr1);

    if (enc3 != 64) {
      output = output + String.fromCharCode(chr2);
    }
    if (enc4 != 64) {
      output = output + String.fromCharCode(chr3);
    }
  }

  output = utf8_decode(output);

  return output;
};

// private method for UTF-8 encoding
const utf8_encode = (str: string) => {
  str = str.replace(/\r\n/g, "\n");
  let utftext = "";

  for (let n = 0; n < str.length; n++) {
    const c = str.charCodeAt(n);

    if (c < 128) {
      utftext += String.fromCharCode(c);
    } else if (c > 127 && c < 2048) {
      utftext += String.fromCharCode((c >> 6) | 192);
      utftext += String.fromCharCode((c & 63) | 128);
    } else {
      utftext += String.fromCharCode((c >> 12) | 224);
      utftext += String.fromCharCode(((c >> 6) & 63) | 128);
      utftext += String.fromCharCode((c & 63) | 128);
    }
  }
  return utftext;
};

// private method for UTF-8 decoding
const utf8_decode = (utftext: string): string => {
  let str = "";
  let i = 0;

  while (i < utftext.length) {
    const c = utftext.charCodeAt(i);

    if (c < 128) {
      str += String.fromCharCode(c);
      i++;
    } else if (c > 191 && c < 224) {
      const c2 = utftext.charCodeAt(i + 1);
      str += String.fromCharCode(((c & 31) << 6) | (c2 & 63));
      i += 2;
    } else {
      const c2 = utftext.charCodeAt(i + 1);
      const c3 = utftext.charCodeAt(i + 2);
      str += String.fromCharCode(((c & 15) << 12) | ((c2 & 63) << 6) | (c3 & 63));
      i += 3;
    }
  }
  return str;
};
