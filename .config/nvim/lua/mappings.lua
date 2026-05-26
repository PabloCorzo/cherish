require "nvchad.mappings"

-- add yours here

local map = vim.keymap.set

map("n", ";", ":", { desc = "CMD enter command mode" })
map("i", "jk", "<ESC>")

-- map({ "n", "i", "v" }, "<C-s>", "<cmd> w <cr>")

map("n", "<C-BS>", "db", { desc = "Delete word backward" })
map("i", "<C-BS>", "<C-o>db", { desc = "Delete word backward" })
