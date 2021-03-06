#pragma once
#include <linux/completion.h>
#include <linux/types.h>

enum {
  NR_MAX_REQ_BIO = 256,
};

struct ssd_info;
struct vdevice_t;
struct fb_lbs_mngr_t;
struct fb_wb;
struct fb_bio_t;
struct fb_context_t;
struct page_mapping_context_t;

struct fb_bio_t {
  u32 req_count;
  u32 lpas[NR_MAX_REQ_BIO];
  u8 *kpages[NR_MAX_REQ_BIO];
  struct bio *bio;

  // Initial state (state after `fb_build_bio()` call) of this completion is
  // "done"
  struct completion bio_lock;
};

struct fb_context_t {
  int err;

  struct page_mapping_context_t *ptr_mapping_context;

  struct fb_lbs_mngr_t *ptr_lbs_mngr;

  struct ssd_info *ptr_ssd_info;
  struct vdevice_t *ptr_vdevice;

  struct gendisk *gd;
  struct request_queue *ptr_req_queue;
  struct completion dev_lock;
  int device_major_num;

  struct fb_wb *wb;  // write_buffer
  struct task_struct *ptr_wb_task;
  u32 flag_enable_wb_thread;

  u64 background_gc_time_stamp;
};

u32 dec_bio_req_count(struct fb_bio_t *ptr_bio);
struct ssd_info *get_ssd_inf(struct fb_context_t *fb);
struct vdevice_t *get_vdev(struct fb_context_t *fb);
struct page_mapping_context_t *get_ftl(struct fb_context_t *fb);
