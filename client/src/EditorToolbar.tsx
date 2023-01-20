import FormatSize from "@mui/icons-material/FormatSize";
import ArrowDropDown from "@mui/icons-material/ArrowDropDown";
import { styled } from "@mui/material/styles";
import Button from "@mui/material/Button";
import ButtonGroup from "@mui/material/ButtonGroup";
import ToggleButton from "@mui/material/ToggleButton";
import Paper from "@mui/material/Paper";
import { ReactComponent as VimIcon } from "./assets/vim.svg";
import React from "react";
import { bindMenu, bindTrigger } from "material-ui-popup-state";
import Menu from "@mui/material/Menu";
import MenuItem from "@mui/material/MenuItem";
import { usePopupState } from "material-ui-popup-state/hooks";
import MenuList from "@mui/material/MenuList";
import { Share } from "@mui/icons-material";

const EditorButtonGroup = styled(ButtonGroup)(({ theme }) => ({
  "& .MuiButtonGroup-grouped": {
    border: 0,
    color: "inherit",
    "&:hover": {
      border: 0,
    },
  },
  "& .MuiToggleButton-root": {
    margin: 0,
    border: 0,
    color: "inherit",
    textTransform: "none",
    "&.Mui-selected": {
      color: theme.palette.primary.main,
    },
  },
}));

interface IEditorToolbarProps {
  vimSelected?: boolean;
  onVimChange?: (event: React.MouseEvent<HTMLElement>, value: any) => void;
  onFontChange?: (size: number) => void;
  onShareClick?: () => void;
  selectedFontSize?: number;
}

function EditorToolbar({
  vimSelected,
  onVimChange,
  onFontChange,
  onShareClick,
  selectedFontSize,
}: IEditorToolbarProps) {
  const popupState = usePopupState({ variant: "popover", popupId: "demoMenu" });

  const fontSizes = [...Array(23).keys()].map((i) => i + 8);

  const handleFontSizeClick = (event: any) => {
    onFontChange && onFontChange(event.target.value);
    popupState.close();
  };

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
        <React.Fragment>
          <Button {...bindTrigger(popupState)}>
            <FormatSize />
            <ArrowDropDown />
          </Button>
          <Menu {...bindMenu(popupState)}>
            <MenuList dense disablePadding>
              {fontSizes.map((size) => (
                <MenuItem
                  key={size}
                  value={size}
                  onClick={handleFontSizeClick}
                  selected={size === selectedFontSize}
                >
                  {size}
                </MenuItem>
              ))}
            </MenuList>
          </Menu>
        </React.Fragment>
        <Button onClick={onShareClick} startIcon={<Share />}>
          Share
        </Button>
        <ToggleButton value="vim" selected={vimSelected} onChange={onVimChange}>
          <VimIcon style={{ marginRight: "8px" }} />
          Vim
        </ToggleButton>
      </EditorButtonGroup>
    </Paper>
  );
}

export default EditorToolbar;
