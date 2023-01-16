import axios from "axios";
import { editor } from "monaco-editor";

namespace api {
  export type Status = "safe" | "unsafe" | "error";
  export interface VerifyRes {
    status: Status;
    markers: editor.IMarkerData[];
  }

  export interface FatalError {
    error: string;
  }

  export const verify = (code: string): Promise<api.VerifyRes | api.FatalError> => {
    const req = { code, crateType: "rlib" };
    return axios
      .post("http://localhost:3000/api/verify", req)
      .then((response) => response.data)
      .catch((response) => {
        if (response.response.status == 500) {
          return response.response.data;
        } else {
          return {
            error: response.response.statusText,
          };
        }
      });
  };
}

export default api;
