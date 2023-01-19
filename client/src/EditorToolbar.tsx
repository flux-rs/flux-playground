import FormatSize from "@mui/icons-material/FormatSize";
import ArrowDropDown from "@mui/icons-material/ArrowDropDown";
import { styled } from "@mui/material/styles";
import Button from "@mui/material/Button";
import ButtonGroup from "@mui/material/ButtonGroup";
import ToggleButton from "@mui/material/ToggleButton";
import Paper from "@mui/material/Paper";
import { ReactComponent as VimIcon } from "./assets/vim.svg";
import { useState } from "react";

const EditorButtonGroup = styled(ButtonGroup)(({ theme }) => ({
  "& .MuiButtonGroup-grouped": {
    border: 0,
  },
  "& .MuiToggleButton-root": {
    margin: 0,
    border: 0,
    textTransform: "none",
  },
}));

function EditorToolbar() {
  const [selected, setSelected] = useState(false);

  return (
    <Paper
      elevation={0}
      sx={{
        border: (theme) => `1px solid ${theme.palette.divider}`,
        borderBottomWidth: "0",
        borderBottomLeftRadius: "0",
        borderBottomRightRadius: "0",
      }}
    >
      <EditorButtonGroup>
        <Button>
          <FormatSize />
          <ArrowDropDown />
        </Button>
        <ToggleButton value="vim" selected={selected} onChange={() => setSelected(!selected)}>
          <VimIcon />
          Vim
        </ToggleButton>
      </EditorButtonGroup>
    </Paper>
  );
}

export default EditorToolbar;
