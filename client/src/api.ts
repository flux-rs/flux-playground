import axios from "axios";
import { editor } from "monaco-editor";

const BASE_URL = import.meta.env.VITE_API_BASE_URL;

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
      .post(`${BASE_URL}/verify`, req)
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
