-- ─────────────────────────────────────────────────────────────
-- Neovim config — minimal, functional
-- ─────────────────────────────────────────────────────────────

-- Disable netrw before anything loads (required by nvim-tree)
vim.g.loaded_netrw       = 1
vim.g.loaded_netrwPlugin = 1

-- Bootstrap lazy.nvim
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.uv.fs_stat(lazypath) then
  vim.fn.system({ "git", "clone", "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git", "--branch=stable", lazypath })
end
vim.opt.rtp:prepend(lazypath)

-- Leader (must be set before plugins load)
vim.g.mapleader      = " "
vim.g.maplocalleader = " "

-- ── Options ──────────────────────────────────────────────────
local o = vim.opt
o.number         = true
o.relativenumber = true
o.tabstop        = 2
o.shiftwidth     = 2
o.expandtab      = true
o.smartindent    = true
o.wrap           = false
o.termguicolors  = true
o.signcolumn     = "yes"
o.updatetime     = 250
o.timeoutlen     = 300    -- ms before which-key popup appears
o.clipboard      = "unnamedplus"
o.cursorline     = true
o.scrolloff      = 8

-- ── vim-visual-multi: free <C-n>, use Shift+Alt+arrows for cursors ─
vim.g.VM_default_mappings = 1
vim.g.VM_maps = {
  ["Find Under"]         = "<C-d>",       -- <C-n> freed for file tree
  ["Find Subword Under"] = "<C-d>",
  ["Add Cursor Down"]    = "<S-A-Down>",
  ["Add Cursor Up"]      = "<S-A-Up>",
}

-- ── Plugins ──────────────────────────────────────────────────
require("lazy").setup({

  -- File tree
  {
    "nvim-tree/nvim-tree.lua",
    dependencies = { "nvim-tree/nvim-web-devicons" },
    config = function()
      require("nvim-tree").setup({ view = { width = 32 } })
    end,
  },

  -- Fuzzy finder
  {
    "nvim-telescope/telescope.nvim", tag = "0.1.x",
    dependencies = { "nvim-lua/plenary.nvim" },
    config = function()
      require("telescope").setup({
        defaults = { file_ignore_patterns = { "node_modules", ".git/" } },
      })
    end,
  },

  -- Harpoon2: quick-access file marks
  {
    "ThePrimeagen/harpoon",
    branch = "harpoon2",
    dependencies = { "nvim-lua/plenary.nvim" },
    config = function()
      local harpoon = require("harpoon")
      harpoon:setup()
      -- Ctrl+A: append current file to harpoon list
      vim.keymap.set("n", "<C-a>", function() harpoon:list():add() end,
        { desc = "Harpoon: add file" })
      -- Ctrl+E: toggle the harpoon quick-menu
      vim.keymap.set("n", "<C-e>", function() harpoon.ui:toggle_quick_menu(harpoon:list()) end,
        { desc = "Harpoon: open list" })
    end,
  },

  -- Key hint overlay shown after <leader> pause
  {
    "folke/which-key.nvim", event = "VeryLazy",
    config = function()
      local wk = require("which-key")
      wk.setup({ delay = 300 })
      wk.add({
        { "<leader>f", group = "Find" },
        { "<leader>t", group = "Theme" },
        { "<leader>r", group = "Rename" },
        { "<leader>c", group = "Code" },
      })
    end,
  },

  -- Multi-cursor (Shift+Alt+Up/Down spawns cursors)
  { "mg979/vim-visual-multi", branch = "master" },

  -- LSP installer (downloads language servers automatically)
  { "williamboman/mason.nvim", config = true },

  -- LSP config — rust + python
  {
    "neovim/nvim-lspconfig",
    dependencies = { "williamboman/mason-lspconfig.nvim", "hrsh7th/cmp-nvim-lsp" },
    config = function()
      require("mason-lspconfig").setup({
        ensure_installed = { "rust_analyzer", "pyright" },
      })
      -- nvim 0.11+ API: avoids the deprecated require('lspconfig').server.setup() call
      -- "*" applies capabilities to every server; lspconfig still provides default cmd/root_dir/etc.
      local caps = require("cmp_nvim_lsp").default_capabilities()
      vim.lsp.config("*", { capabilities = caps })
      vim.lsp.enable({ "rust_analyzer", "pyright" })
    end,
  },

  -- Completion
  {
    "hrsh7th/nvim-cmp",
    dependencies = {
      "hrsh7th/cmp-nvim-lsp", "hrsh7th/cmp-buffer", "hrsh7th/cmp-path",
      "L3MON4D3/LuaSnip", "saadparwaiz1/cmp_luasnip",
    },
    config = function()
      local cmp = require("cmp")
      local ls  = require("luasnip")
      cmp.setup({
        snippet = { expand = function(a) ls.lsp_expand(a.body) end },
        mapping = cmp.mapping.preset.insert({
          ["<C-Space>"] = cmp.mapping.complete(),
          ["<CR>"]      = cmp.mapping.confirm({ select = true }),
          ["<Tab>"] = cmp.mapping(function(fb)
            if cmp.visible() then cmp.select_next_item()
            elseif ls.expand_or_jumpable() then ls.expand_or_jump()
            else fb() end
          end, { "i", "s" }),
          ["<S-Tab>"] = cmp.mapping(function(fb)
            if cmp.visible() then cmp.select_prev_item()
            elseif ls.jumpable(-1) then ls.jump(-1)
            else fb() end
          end, { "i", "s" }),
        }),
        sources = cmp.config.sources({
          { name = "nvim_lsp" }, { name = "luasnip" },
          { name = "buffer" },   { name = "path" },
        }),
      })
    end,
  },

  -- Rename with live inline preview — type new name, press Enter
  {
    "smjonas/inc-rename.nvim",
    config = function() require("inc_rename").setup() end,
  },

  -- Diagnostics panel (<leader>tt to toggle)
  {
    "folke/trouble.nvim",
    dependencies = { "nvim-tree/nvim-web-devicons" },
    config = function()
      require("trouble").setup()
    end,
  },


  -- ── Themes: brown/warm ───────────────────────────────────────
  { "ellisonleao/gruvbox.nvim",
    config = function() require("gruvbox").setup({ contrast = "hard" }) end },
  { "sainnhe/gruvbox-material" },
  { "rebelot/kanagawa.nvim" },
  { "savq/melange-nvim" },
  -- ── Themes: green ────────────────────────────────────────────
  { "sainnhe/everforest" },
  { "ribru17/bamboo.nvim",
    config = function() require("bamboo").setup({ style = "vulgaris" }) end },

}, {})

-- ── Theme picker (<leader>th) ────────────────────────────────
local themes = {
  { label = "gruvbox          [brown / hard]", apply = function()
    vim.o.background = "dark"; vim.cmd.colorscheme("gruvbox") end },
  { label = "gruvbox-material [brown / soft]", apply = function()
    vim.g.gruvbox_material_background = "soft"
    vim.o.background = "dark"; vim.cmd.colorscheme("gruvbox-material") end },
  { label = "kanagawa-dragon  [earth / warm]", apply = function()
    vim.cmd.colorscheme("kanagawa-dragon") end },
  { label = "melange          [amber / brown]", apply = function()
    vim.o.background = "dark"; vim.cmd.colorscheme("melange") end },
  { label = "everforest       [green / hard]", apply = function()
    vim.g.everforest_background = "hard"
    vim.o.background = "dark"; vim.cmd.colorscheme("everforest") end },
  { label = "bamboo           [green / lush]", apply = function()
    vim.cmd.colorscheme("bamboo") end },
}

local theme_file = vim.fn.stdpath("data") .. "/nvim_theme"

local function pick_theme()
  local labels = vim.tbl_map(function(t) return t.label end, themes)
  vim.ui.select(labels, { prompt = "Pick a theme:" }, function(choice)
    if not choice then return end
    for _, t in ipairs(themes) do
      if t.label == choice then
        t.apply()
        local f = io.open(theme_file, "w")
        if f then f:write(choice); f:close() end
        break
      end
    end
  end)
end

local function load_saved_theme()
  local f = io.open(theme_file, "r")
  if not f then return false end
  local label = f:read("*l"); f:close()
  for _, t in ipairs(themes) do
    if t.label == label then pcall(t.apply); return true end
  end
  return false
end

-- ── Keymaps ──────────────────────────────────────────────────
local map = function(modes, lhs, rhs, desc)
  vim.keymap.set(modes, lhs, rhs, { noremap = true, silent = true, desc = desc })
end

-- File tree
map("n", "<C-n>", "<cmd>NvimTreeToggle<CR>", "Toggle file tree")

-- Ctrl+Backspace → delete word backwards (insert mode)
-- Most terminals send <C-H> (ASCII 8) for Ctrl+Backspace
map("i", "<C-BS>", "<C-w>", "Delete word backwards")
map("i", "<C-H>",  "<C-w>", "Delete word backwards")

-- Alt+Up/Down → move line or selection
map("n", "<A-Up>",   "<cmd>m .-2<CR>==",  "Move line up")
map("n", "<A-Down>", "<cmd>m .+1<CR>==",  "Move line down")
map("v", "<A-Up>",   ":m '<-2<CR>gv=gv",  "Move selection up")
map("v", "<A-Down>", ":m '>+1<CR>gv=gv",  "Move selection down")

-- ; acts as : (enter command mode without Shift)
map("n", ";", ":", "Command mode")

-- Fuzzy find
map("n", "<leader>ff", "<cmd>Telescope find_files<CR>", "Find files")
map("n", "<leader>fg", "<cmd>Telescope live_grep<CR>",  "Live grep")
map("n", "<leader>fb", "<cmd>Telescope buffers<CR>",    "Find buffers")

-- Theme picker
map("n", "<leader>th", pick_theme, "Pick theme")

-- Diagnostics panel (trouble)
map("n", "<leader>l", "<cmd>Trouble diagnostics toggle<cr>", "Toggle diagnostics panel")

-- Rename symbol — opens `:IncRename <word>`, just type new name + Enter
vim.keymap.set("n", "<leader>rn", function()
  return ":IncRename " .. vim.fn.expand("<cword>")
end, { expr = true, desc = "Rename symbol" })

-- Diagnostics
map("n", "[d",        vim.diagnostic.goto_prev,  "Prev diagnostic")
map("n", "]d",        vim.diagnostic.goto_next,  "Next diagnostic")
map("n", "<leader>e", vim.diagnostic.open_float, "Show diagnostic")

-- LSP buffer-local keys (registered when an LSP attaches to a buffer)
vim.api.nvim_create_autocmd("LspAttach", {
  callback = function(ev)
    local bmap = function(lhs, rhs, desc)
      vim.keymap.set("n", lhs, rhs,
        { buffer = ev.buf, noremap = true, silent = true, desc = desc })
    end
    bmap("gd",         vim.lsp.buf.definition,  "Go to definition")
    bmap("gD",         vim.lsp.buf.declaration, "Go to declaration")
    bmap("K",          vim.lsp.buf.hover,       "Hover docs")
    bmap("<leader>ca", vim.lsp.buf.code_action, "Code action")
  end,
})

-- ── Apply colorscheme ────────────────────────────────────────
-- Saved theme → gruvbox default → habamax (first-run fallback before plugins install)
if not load_saved_theme() then
  local ok = pcall(function()
    vim.o.background = "dark"
    vim.cmd.colorscheme("gruvbox")
  end)
  if not ok then vim.cmd.colorscheme("habamax") end
end
