#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 13
#define STATE_COUNT 25
#define LARGE_STATE_COUNT 12
#define SYMBOL_COUNT 17
#define ALIAS_COUNT 0
#define TOKEN_COUNT 10
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 4
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 3

enum {
  aux_sym__statement_token1 = 1,
  anon_sym_STAR = 2,
  anon_sym_SLASH = 3,
  anon_sym_PLUS = 4,
  anon_sym_DASH = 5,
  sym_constant = 6,
  anon_sym_LPAREN = 7,
  anon_sym_RPAREN = 8,
  sym_comment = 9,
  sym_source_file = 10,
  sym__statement = 11,
  sym__expression = 12,
  sym_binary_expression = 13,
  sym_unary_expression = 14,
  sym_paren_expression = 15,
  aux_sym_source_file_repeat1 = 16,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [aux_sym__statement_token1] = "_statement_token1",
  [anon_sym_STAR] = "*",
  [anon_sym_SLASH] = "/",
  [anon_sym_PLUS] = "+",
  [anon_sym_DASH] = "-",
  [sym_constant] = "constant",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [sym_comment] = "comment",
  [sym_source_file] = "source_file",
  [sym__statement] = "_statement",
  [sym__expression] = "_expression",
  [sym_binary_expression] = "binary_expression",
  [sym_unary_expression] = "unary_expression",
  [sym_paren_expression] = "paren_expression",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [aux_sym__statement_token1] = aux_sym__statement_token1,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_DASH] = anon_sym_DASH,
  [sym_constant] = sym_constant,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_comment] = sym_comment,
  [sym_source_file] = sym_source_file,
  [sym__statement] = sym__statement,
  [sym__expression] = sym__expression,
  [sym_binary_expression] = sym_binary_expression,
  [sym_unary_expression] = sym_unary_expression,
  [sym_paren_expression] = sym_paren_expression,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [aux_sym__statement_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [sym_constant] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__statement] = {
    .visible = false,
    .named = true,
  },
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [sym_binary_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_unary_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_paren_expression] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_left = 1,
  field_operand = 2,
  field_operator = 3,
  field_right = 4,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_left] = "left",
  [field_operand] = "operand",
  [field_operator] = "operator",
  [field_right] = "right",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_operand, 1},
    {field_operator, 0},
  [2] =
    {field_left, 0},
    {field_operator, 1},
    {field_right, 2},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(6);
      if (lookahead == '#') ADVANCE(22);
      if (lookahead == '(') ADVANCE(20);
      if (lookahead == ')') ADVANCE(21);
      if (lookahead == '*') ADVANCE(8);
      if (lookahead == '+') ADVANCE(10);
      if (lookahead == '-') ADVANCE(11);
      if (lookahead == '/') ADVANCE(9);
      if (lookahead == '0') ADVANCE(13);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(14);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(7);
      if (lookahead == '#') ADVANCE(22);
      if (lookahead == '*') ADVANCE(8);
      if (lookahead == '+') ADVANCE(10);
      if (lookahead == '-') ADVANCE(11);
      if (lookahead == '/') ADVANCE(9);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      END_STATE();
    case 2:
      if (lookahead == '0') ADVANCE(15);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(16);
      END_STATE();
    case 3:
      if (lookahead == '0') ADVANCE(12);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(18);
      END_STATE();
    case 4:
      if (lookahead == '0' ||
          lookahead == '1') ADVANCE(17);
      END_STATE();
    case 5:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(19);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(aux_sym__statement_token1);
      if (lookahead == '\n') ADVANCE(7);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym_constant);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_constant);
      if (lookahead == '.') ADVANCE(2);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(4);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(3);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(5);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_constant);
      if (lookahead == '.') ADVANCE(2);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(3);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(14);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(sym_constant);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(3);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym_constant);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(3);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(16);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(sym_constant);
      if (lookahead == '0' ||
          lookahead == '1') ADVANCE(17);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym_constant);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(18);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(sym_constant);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(19);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(22);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 1},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 1},
  [18] = {.lex_state = 1},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 1},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 1},
  [24] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [sym_constant] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(24),
    [sym__statement] = STATE(3),
    [sym__expression] = STATE(23),
    [sym_binary_expression] = STATE(23),
    [sym_unary_expression] = STATE(23),
    [sym_paren_expression] = STATE(23),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_PLUS] = ACTIONS(7),
    [anon_sym_DASH] = ACTIONS(7),
    [sym_constant] = ACTIONS(9),
    [anon_sym_LPAREN] = ACTIONS(11),
    [sym_comment] = ACTIONS(3),
  },
  [2] = {
    [sym__statement] = STATE(2),
    [sym__expression] = STATE(23),
    [sym_binary_expression] = STATE(23),
    [sym_unary_expression] = STATE(23),
    [sym_paren_expression] = STATE(23),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(13),
    [anon_sym_PLUS] = ACTIONS(15),
    [anon_sym_DASH] = ACTIONS(15),
    [sym_constant] = ACTIONS(18),
    [anon_sym_LPAREN] = ACTIONS(21),
    [sym_comment] = ACTIONS(3),
  },
  [3] = {
    [sym__statement] = STATE(2),
    [sym__expression] = STATE(23),
    [sym_binary_expression] = STATE(23),
    [sym_unary_expression] = STATE(23),
    [sym_paren_expression] = STATE(23),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(24),
    [anon_sym_PLUS] = ACTIONS(7),
    [anon_sym_DASH] = ACTIONS(7),
    [sym_constant] = ACTIONS(9),
    [anon_sym_LPAREN] = ACTIONS(11),
    [sym_comment] = ACTIONS(3),
  },
  [4] = {
    [sym__expression] = STATE(22),
    [sym_binary_expression] = STATE(22),
    [sym_unary_expression] = STATE(22),
    [sym_paren_expression] = STATE(22),
    [anon_sym_PLUS] = ACTIONS(26),
    [anon_sym_DASH] = ACTIONS(26),
    [sym_constant] = ACTIONS(28),
    [anon_sym_LPAREN] = ACTIONS(30),
    [sym_comment] = ACTIONS(3),
  },
  [5] = {
    [sym__expression] = STATE(21),
    [sym_binary_expression] = STATE(21),
    [sym_unary_expression] = STATE(21),
    [sym_paren_expression] = STATE(21),
    [anon_sym_PLUS] = ACTIONS(7),
    [anon_sym_DASH] = ACTIONS(7),
    [sym_constant] = ACTIONS(32),
    [anon_sym_LPAREN] = ACTIONS(11),
    [sym_comment] = ACTIONS(3),
  },
  [6] = {
    [sym__expression] = STATE(15),
    [sym_binary_expression] = STATE(15),
    [sym_unary_expression] = STATE(15),
    [sym_paren_expression] = STATE(15),
    [anon_sym_PLUS] = ACTIONS(26),
    [anon_sym_DASH] = ACTIONS(26),
    [sym_constant] = ACTIONS(34),
    [anon_sym_LPAREN] = ACTIONS(30),
    [sym_comment] = ACTIONS(3),
  },
  [7] = {
    [sym__expression] = STATE(20),
    [sym_binary_expression] = STATE(20),
    [sym_unary_expression] = STATE(20),
    [sym_paren_expression] = STATE(20),
    [anon_sym_PLUS] = ACTIONS(26),
    [anon_sym_DASH] = ACTIONS(26),
    [sym_constant] = ACTIONS(36),
    [anon_sym_LPAREN] = ACTIONS(30),
    [sym_comment] = ACTIONS(3),
  },
  [8] = {
    [sym__expression] = STATE(19),
    [sym_binary_expression] = STATE(19),
    [sym_unary_expression] = STATE(19),
    [sym_paren_expression] = STATE(19),
    [anon_sym_PLUS] = ACTIONS(26),
    [anon_sym_DASH] = ACTIONS(26),
    [sym_constant] = ACTIONS(38),
    [anon_sym_LPAREN] = ACTIONS(30),
    [sym_comment] = ACTIONS(3),
  },
  [9] = {
    [sym__expression] = STATE(12),
    [sym_binary_expression] = STATE(12),
    [sym_unary_expression] = STATE(12),
    [sym_paren_expression] = STATE(12),
    [anon_sym_PLUS] = ACTIONS(26),
    [anon_sym_DASH] = ACTIONS(26),
    [sym_constant] = ACTIONS(40),
    [anon_sym_LPAREN] = ACTIONS(30),
    [sym_comment] = ACTIONS(3),
  },
  [10] = {
    [sym__expression] = STATE(17),
    [sym_binary_expression] = STATE(17),
    [sym_unary_expression] = STATE(17),
    [sym_paren_expression] = STATE(17),
    [anon_sym_PLUS] = ACTIONS(7),
    [anon_sym_DASH] = ACTIONS(7),
    [sym_constant] = ACTIONS(42),
    [anon_sym_LPAREN] = ACTIONS(11),
    [sym_comment] = ACTIONS(3),
  },
  [11] = {
    [sym__expression] = STATE(14),
    [sym_binary_expression] = STATE(14),
    [sym_unary_expression] = STATE(14),
    [sym_paren_expression] = STATE(14),
    [anon_sym_PLUS] = ACTIONS(7),
    [anon_sym_DASH] = ACTIONS(7),
    [sym_constant] = ACTIONS(44),
    [anon_sym_LPAREN] = ACTIONS(11),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(50), 1,
      anon_sym_RPAREN,
    ACTIONS(46), 2,
      anon_sym_STAR,
      anon_sym_SLASH,
    ACTIONS(48), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [15] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(52), 5,
      anon_sym_STAR,
      anon_sym_SLASH,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_RPAREN,
  [26] = 4,
    ACTIONS(54), 1,
      aux_sym__statement_token1,
    ACTIONS(60), 1,
      sym_comment,
    ACTIONS(56), 2,
      anon_sym_STAR,
      anon_sym_SLASH,
    ACTIONS(58), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [41] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(62), 5,
      anon_sym_STAR,
      anon_sym_SLASH,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_RPAREN,
  [52] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(64), 5,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_DASH,
      sym_constant,
      anon_sym_LPAREN,
  [63] = 3,
    ACTIONS(54), 1,
      aux_sym__statement_token1,
    ACTIONS(60), 1,
      sym_comment,
    ACTIONS(58), 4,
      anon_sym_STAR,
      anon_sym_SLASH,
      anon_sym_PLUS,
      anon_sym_DASH,
  [76] = 3,
    ACTIONS(52), 1,
      aux_sym__statement_token1,
    ACTIONS(60), 1,
      sym_comment,
    ACTIONS(66), 4,
      anon_sym_STAR,
      anon_sym_SLASH,
      anon_sym_PLUS,
      anon_sym_DASH,
  [89] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(54), 5,
      anon_sym_STAR,
      anon_sym_SLASH,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_RPAREN,
  [100] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(46), 2,
      anon_sym_STAR,
      anon_sym_SLASH,
    ACTIONS(54), 3,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_RPAREN,
  [113] = 3,
    ACTIONS(60), 1,
      sym_comment,
    ACTIONS(62), 1,
      aux_sym__statement_token1,
    ACTIONS(68), 4,
      anon_sym_STAR,
      anon_sym_SLASH,
      anon_sym_PLUS,
      anon_sym_DASH,
  [126] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(70), 1,
      anon_sym_RPAREN,
    ACTIONS(46), 2,
      anon_sym_STAR,
      anon_sym_SLASH,
    ACTIONS(48), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [141] = 4,
    ACTIONS(60), 1,
      sym_comment,
    ACTIONS(72), 1,
      aux_sym__statement_token1,
    ACTIONS(56), 2,
      anon_sym_STAR,
      anon_sym_SLASH,
    ACTIONS(74), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [156] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(76), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(12)] = 0,
  [SMALL_STATE(13)] = 15,
  [SMALL_STATE(14)] = 26,
  [SMALL_STATE(15)] = 41,
  [SMALL_STATE(16)] = 52,
  [SMALL_STATE(17)] = 63,
  [SMALL_STATE(18)] = 76,
  [SMALL_STATE(19)] = 89,
  [SMALL_STATE(20)] = 100,
  [SMALL_STATE(21)] = 113,
  [SMALL_STATE(22)] = 126,
  [SMALL_STATE(23)] = 141,
  [SMALL_STATE(24)] = 156,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [15] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(5),
  [18] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(23),
  [21] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(9),
  [24] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [26] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [28] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [30] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [32] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [34] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [36] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [38] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [40] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [42] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [44] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [48] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_expression, 3),
  [54] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_expression, 3, .production_id = 2),
  [56] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [58] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_expression, 3, .production_id = 2),
  [60] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_expression, 2, .production_id = 1),
  [64] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__statement, 2),
  [66] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_paren_expression, 3),
  [68] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_expression, 2, .production_id = 1),
  [70] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [74] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [76] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_calc(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
