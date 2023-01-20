import VerifyButton from "./VerifyButton";
import React, { useRef, MutableRefObject, useState, useEffect } from "react";
import Box from "@mui/material/Box";
import Container from "@mui/material/Container";
import ResultAlert from "./ResultAlert";
import api from "./api";
import FatalError from "./FatalError";
import EditorToolbar from "./EditorToolbar";
import ListSubheader from "@mui/material/ListSubheader";
import MenuItem from "@mui/material/MenuItem";
import Stack from "@mui/material/Stack";
import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import Select from "@mui/material/Select";
import Paper from "@mui/material/Paper";
import Editor, { Monaco } from "@monaco-editor/react";
import { editor } from "monaco-editor";
import { useLocation, Link as RouterLink } from "react-router-dom";
import Link from "@mui/material/Link";

type IStandaloneCodeEditor = editor.IStandaloneCodeEditor;
type IMarkerData = editor.IMarkerData;

interface IExamplesMap {
  [key: string]: { groupName: string; examples: api.IExample[] };
}

function App() {
  const value = `#![allow(unused)]

#[flux::sig(fn() -> i32[10])]
fn mk_ten() -> i32 {
    4 + 5
}
`;
  const monacoDefaultOptions: editor.IStandaloneEditorConstructionOptions = {
    minimap: { enabled: false },
    fontSize: 14,
  };

  const [monacoOptions, setMonacoOptions] = useState(monacoDefaultOptions);
  const [fatalError, setFatalError] = useState(undefined as string | undefined);
  const [status, setStatus] = useState(undefined as api.Status | undefined);
  const [verifying, setVerifying] = useState(false);
  const [selectedExample, setSelectedExample] = useState("");
  const [examples, setExamples] = useState({} as IExamplesMap);
  const [vimSelected, setVimSelected] = useState(false);
  const search = useLocation().search;

  const monacoRef: MutableRefObject<Monaco | null> = useRef(null);
  const editorRef: MutableRefObject<IStandaloneCodeEditor | null> = useRef(null);
  const vimModeRef: MutableRefObject<any> = useRef(null);
  const vimStatusBarRef: MutableRefObject<HTMLDivElement | null> = useRef(null);

  const resizeEditor = () => {
    editorRef.current?.layout({} as editor.IDimension);
  };

  useEffect(() => {
    api.listExamples().then((response) => {
      if ("error" in response) {
        setFatalError(response.error);
      } else {
        const map: IExamplesMap = {};
        for (const group of response.groups) {
          map[group.id] = { groupName: group.displayName, examples: [] };
        }
        for (const example of response.examples) {
          map[example.groupId]?.examples.push(example);
        }
        setExamples(map);
      }
    });

    window.addEventListener("resize", resizeEditor);
    return () => {
      window.removeEventListener("resize", () => {});
    };
  }, []);

  useEffect(() => {
    const editor = editorRef.current;
    const example = new URLSearchParams(search).get("example");
    editor && example && loadExample(editor, example);
  }, [search]);

  const doVerify = () => {
    if (verifying) return;
    const code = editorRef.current?.getValue();
    if (code) {
      setVerifying(true);
      api.verify(code).then((response) => {
        setVerifying(false);
        if ("error" in response) {
          setFatalError(response.error);
        } else {
          setStatusAndMarkers(response.status, response.markers);
        }
      });
    }
  };

  const setStatusAndMarkers = (status: api.Status | undefined, markers: IMarkerData[]) => {
    const monaco = monacoRef.current;
    const model = editorRef.current?.getModel();
    if (monaco && model) {
      monaco.editor.setModelMarkers(model, "owner", markers);
      setStatus(status);
    }
  };

  const editorDidMount = (editor: IStandaloneCodeEditor, monaco: Monaco) => {
    monacoRef.current = monaco;
    editorRef.current = editor;

    const example = new URLSearchParams(search).get("example");
    editor && example && loadExample(editor, example);

    (window.require as any).config({
      paths: {
        "monaco-vim": "https://unpkg.com/monaco-vim@0.3.5/dist/monaco-vim.js",
      },
    });
  };

  const loadExample = (editor: IStandaloneCodeEditor, example: string) => {
    api.getExampleCode(example).then((response) => {
      setStatusAndMarkers(undefined, []);
      if ("error" in response) {
        setFatalError(response.error);
        editor.setValue("");
      } else {
        editor.setValue(response.code);
      }
    });
  };

  const closeFatalError = (_event?: React.SyntheticEvent | Event, _reason?: string) => {
    setFatalError(undefined);
  };

  const closeResult = (_event: React.SyntheticEvent) => {
    setStatusAndMarkers(undefined, []);
  };

  const toggleVim = () => {
    const editor = editorRef.current;
    if (!editor) return;
    vimModeRef.current?.dispose();
    if (vimSelected) {
      setVimSelected(false);
    } else {
      (window.require as any)(["monaco-vim"], (MonacoVim: any) => {
        setVimSelected(true);
        vimModeRef.current = MonacoVim.initVimMode(editor, vimStatusBarRef.current);
        resizeEditor();
      });
    }
  };

  const onFontChange = (fontSize: number) => {
    setMonacoOptions({ ...monacoOptions, fontSize });
  };

  const exampleItems = [];
  let key = 0;
  for (const group of Object.values(examples)) {
    exampleItems.push(<ListSubheader key={key++}>{group.groupName}</ListSubheader>);
    for (const example of group.examples) {
      exampleItems.push(
        <MenuItem key={key++} value="">
          <Link component={RouterLink} to={`?example=${example.fileName}`}>
            {example.displayName}
          </Link>
        </MenuItem>
      );
    }
  }

  return (
    <Container className="content" maxWidth={false} sx={{ height: "100vh" }}>
      <Stack height="100%">
        <h1>Flux Playground</h1>
        <Box>
          <FormControl sx={{ minWidth: 160 }}>
            <InputLabel>Select Example</InputLabel>
            <Select label="Select Example" value={selectedExample}>
              {exampleItems.map((item) => item)}
            </Select>
          </FormControl>
        </Box>
        <Stack direction="row" spacing={2} sx={{ padding: "1.25em 0", minHeight: "48px" }}>
          <VerifyButton onClick={doVerify} verifying={verifying} />
          <ResultAlert status={status} onClose={closeResult} />
        </Stack>
        <EditorToolbar
          vimSelected={vimSelected}
          onVimChange={toggleVim}
          onFontChange={onFontChange}
          selectedFontSize={monacoOptions.fontSize}
        />
        <Box
          flex={1}
          sx={{
            border: (theme) => `1px solid ${theme.palette.divider}`,
          }}
        >
          <Editor
            className="editor"
            value={value}
            language="rust"
            options={monacoOptions}
            onMount={editorDidMount}
          />
        </Box>
        <Paper
          id="vim-status-bar"
          ref={vimStatusBarRef}
          elevation={0}
          square
          sx={{
            backgroundColor: "rgb(0, 122, 204)",
            paddingLeft: "10px",
            color: "white",
            fontFamily: '"Droid Sans Mono", "monospace", monospace',
          }}
        ></Paper>
        <FatalError message={fatalError} onClose={closeFatalError}></FatalError>
      </Stack>
    </Container>
  );
}

export default App;
