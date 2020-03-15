#pragma once
#include "rust/libflashbench.h"

enum {
  //
  // Parameterf for DEL manager
  //
  NR_MAX_LPAS_DISCARD = 2048,
  NUM_WTODS = NR_MAX_LPAS_DISCARD,
  NUM_BTODS = NR_MAX_LPAS_DISCARD,
  NR_MAX_LPGS_COPY = NR_MAX_LPAS_DISCARD * 3,

  //
  // Hardware configuration
  //
  CFACTOR_PERCENT = 90,
  NR_LP_IN_PP = 4,
  LP_PAGE_MASK = 0x3,
  LP_PAGE_SHIFT = 2,

  NUM_CHIPS = NUM_BUSES * NUM_CHIPS_PER_BUS,
  NUM_BLOCKS = NUM_CHIPS * NUM_BLOCKS_PER_CHIP,
  NUM_PAGES = NUM_BLOCKS * NUM_PAGES_PER_BLOCK,
  NUM_PAGES_IN_WRITE_BUFFER = NR_LP_IN_PP * NUM_CHIPS,
  NUM_LOG_PAGES = NUM_PAGES * NR_LP_IN_PP * CFACTOR_PERCENT / 100,

  LOGICAL_PAGE_SIZE = 4096,
  PHYSICAL_PAGE_SIZE = LOGICAL_PAGE_SIZE * NR_LP_IN_PP,
  SECTOR_SIZE = 512,

  TREAD = 80,
  TPROG = 800,
  TBERS = 5000,
  TPLOCK = 100,
  TBLOCK = 100,

  NUM_MAX_ENTRIES_OPR_QUEUE = 4,

  BGC_TH_INTV = 5000,
  BGC_TH_WB_UTIL = 5,
  BGC_TH_NR_BLKS = 14,
};
