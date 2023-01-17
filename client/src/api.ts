import axios from "axios";
import { editor } from "monaco-editor";

const BASE_URL = import.meta.env.VITE_BASE_URL;
const API_BASE_URL = `${BASE_URL}/api`;

namespace api {
  export type Status = "safe" | "unsafe" | "error";

  export interface IVerifyResponse {
    status: Status;
    markers: editor.IMarkerData[];
  }

  export interface IListExamplesResponse {
    groups: IExampleGroup[];
    examples: IExample[];
  }

  export interface IExampleGroup {
    displayName: string;
    id: string;
  }

  export interface IExample {
    displayName: string;
    fileName: string;
    groupId: string;
  }

  export interface IExampleCode {
    code: string;
  }

  export interface IFatalError {
    error: string;
  }

  export const verify = (code: string): Promise<api.IVerifyResponse | api.IFatalError> => {
    const req = { code, crateType: "rlib" };
    return axios
      .post(`${API_BASE_URL}/verify`, req)
      .then((response) => response.data)
      .catch(mapErr);
  };

  export const listExamples = (): Promise<api.IListExamplesResponse | api.IFatalError> => {
    return axios
      .get(`${API_BASE_URL}/examples`)
      .then((response) => response.data)
      .catch(mapErr);
  };

  export const getExampleCode = (fileName: string): Promise<IExampleCode | api.IFatalError> => {
    return axios
      .get(`${BASE_URL}/examples/${fileName}`)
      .then((response) => {
        return {
          code: response.data,
        };
      })
      .catch(mapErr);
  };

  const mapErr = (response: any): api.IFatalError => {
    if (response.response.status == 500) {
      return response.response.data;
    } else {
      return {
        error: response.response.statusText,
      };
    }
  };
}

export default api;
