use std::libc::{c_uint, c_float, c_ulong, c_ushort, c_short, c_char, c_int, c_void, c_double, c_uchar};

#[repr(C)]
pub struct libraw_decoder_info_t {
	pub decoder_name: *const;
	pub decoder_flags: c_uint;
}

#[repr(C)]
pub struct libraw_internal_output_params_t {
	pub mix_green: c_uint;
	pub raw_color: c_uint;
	pub zero_is_bad: c_uint;
	pub shrink: c_ushort;
	pub fuji_width: c_ushort;
}

#[repr(C)]
pub struct libraw_callbacks_t {
	pub mem_cb: memory_callback;
	pub memcb_data: void*;
	pub data_cb: data_callback;
	pub datacb_data: void*;
	pub progress_cb: progress_callback;
	pub progresscb_data: *c_void;
	pub exif_cb: exif_parser_callback;
	pub exifparser_data: *c_void;
}

#[repr(C)]
pub struct libraw_processed_image_t {
	pub LibRaw_image_formatstype: enum;
	pub height: c_ushort;
	pub width: c_ushort;
	pub colors: c_ushort;
	pub bits: c_ushort;
	pub data_size: c_uint;
	pub data: [c_uchar; 1];
}

#[repr(C)]
pub struct libraw_iparams_t {
	pub make: [c_char; 64];
	pub model: [c_char; 64];
	pub software: [c_char; 64];
	pub raw_count: c_uint;
	pub dng_version: c_uint;
	pub is_foveon: c_uint;
	pub colors: c_int;
	pub filters: c_uint;
	pub xtrans: [[c_char; 6]; 6];
	pub xtrans_abs: [[c_char; 6]; 6];
	pub cdesc: [c_char; 5];
	pub xmplen: c_uint;
	pub xmpdata: *c_char;
}

#[repr(C)]
pub struct libraw_image_sizes_t {
	pub raw_height: c_ushort;
	pub raw_width: c_ushort;
	pub height: c_ushort;
	pub width: c_ushort;
	pub top_margin: c_ushort;
	pub left_margin: c_ushort;
	pub iheight: c_ushort;
	pub iwidth: c_ushort;
	pub raw_pitch: c_uint;
	pub pixel_aspect: c_double;
	pub flip: c_int;
	pub mask: [[c_int; 4]; 8];
}

#[repr(C)]
pub struct libraw_dng_color_t {
	pub illuminant: c_ushort;
	pub calibration: [[c_float; 4]; 4];
	pub colormatrix: [[c_float; 3]; 4];
}

#[repr(C)]
pub struct canon_makernotes_t {
	pub CanonColorDataVer: c_int;
	pub CanonColorDataSubVer: c_int;
	pub SpecularWhiteLevel: c_int;
	pub AverageBlackLevel: c_int;
}

#[repr(C)]
pub struct libraw_colordata_t {
	pub curve: [c_ushort; 0x10000];
	pub cblack: [c_uint; 4102];
	pub black: c_uint;
	pub data_maximum: c_uint;
	pub maximum: c_uint;
	pub white: [[c_ushort; 8]; 8];
	pub cam_mul: [c_float; 4];
	pub pre_mul: [c_float; 4];
	pub cmatrix: [[c_float; 4]; 3];
	pub rgb_cam: [[c_float; 4]; 3];
	pub cam_xyz: [[c_float; 3]; 4];
	pub ph1_tphase_one_data: struct;
	pub flash_used: c_float;
	pub canon_ev: c_float;
	pub model2: [c_char; 64];
	pub profile: *c_void;
	pub profile_length: c_uint;
	pub black_stat: [c_uint; 8];
	pub dng_color: [libraw_dng_color_t; 2];
	pub canon_makernotes: canon_makernotes_t;
	pub baseline_exposure: c_float;
	pub OlympusSensorCalibration: [c_int; 2];
	pub FujiExpoMidPointShift: c_float;
	pub digitalBack_color: c_int;
}

#[repr(C)]
pub struct libraw_thumbnail_t {
	pub LibRaw_thumbnail_formatstformat: enum;
	pub twidth: c_ushort;
	pub theight: c_ushort;
	pub tlength: c_uint;
	pub tcolors: c_int;
	pub thumb: *c_char;
}

#[repr(C)]
pub struct libraw_gps_info_t {
	pub latitude: [c_float; 3];
	pub longtitude: [c_float; 3];
	pub gpstimestamp: [c_float; 3];
	pub altitude: c_float;
	pub altref: c_char;
	pub latref: c_char;
	pub longref: c_char;
	pub gpsstatus: c_char;
	pub gpsparsed: c_char;
}

#[repr(C)]
pub struct libraw_imgother_t {
	pub iso_speed: c_float;
	pub shutter: c_float;
	pub aperture: c_float;
	pub focal_len: c_float;
	pub timestamp: time_t;
	pub shot_order: c_uint;
	pub gpsdata: [c_uint; 32];
	pub parsed_gps: libraw_gps_info_t;
	pub desc: [c_char; 512];
	pub artist: [c_char; 64];
}

#[repr(C)]
pub struct libraw_output_params_t {
	pub greybox: [c_uint; 4];
	pub cropbox: [c_uint; 4];
	pub aber: [c_double; 4];
	pub gamm: [c_double; 6];
	pub user_mul: [c_float; 4];
	pub shot_select: c_uint;
	pub bright: c_float;
	pub threshold: c_float;
	pub half_size: c_int;
	pub four_color_rgb: c_int;
	pub highlight: c_int;
	pub use_auto_wb: c_int;
	pub use_camera_wb: c_int;
	pub use_camera_matrix: c_int;
	pub output_color: c_int;
	pub output_profile: *c_char;
	pub camera_profile: *c_char;
	pub bad_pixels: *c_char;
	pub dark_frame: *c_char;
	pub output_bps: c_int;
	pub output_tiff: c_int;
	pub user_flip: c_int;
	pub user_qual: c_int;
	pub user_black: c_int;
	pub user_cblack: [c_int; 4];
	pub user_sat: c_int;
	pub med_passes: c_int;
	pub auto_bright_thr: c_float;
	pub adjust_maximum_thr: c_float;
	pub no_auto_bright: c_int;
	pub use_fuji_rotate: c_int;
	pub green_matching: c_int;
	pub dcb_iterations: c_int;
	pub dcb_enhance_fl: c_int;
	pub fbdd_noiserd: c_int;
	pub eeci_refine: c_int;
	pub es_med_passes: c_int;
	pub ca_correc: c_int;
	pub cared: c_float;
	pub cablue: c_float;
	pub cfaline: c_int;
	pub linenoise: c_float;
	pub cfa_clean: c_int;
	pub lclean: c_float;
	pub cclean: c_float;
	pub cfa_green: c_int;
	pub green_thresh: c_float;
	pub exp_correc: c_int;
	pub exp_shift: c_float;
	pub exp_preser: c_float;
	pub wf_debanding: c_int;
	pub wf_deband_treshold: [c_float; 4];
	pub use_rawspeed: c_int;
	pub no_auto_scale: c_int;
	pub no_interpolation: c_int;
	pub sraw_ycc: c_int;
	pub force_foveon_x3f: c_int;
	pub x3f_flags: c_int;
	pub sony_arw2_options: c_int;
	pub sony_arw2_posterization_thr: c_int;
	pub coolscan_nef_gamma: c_float;
}

#[repr(C)]
pub struct libraw_rawdata_t {
	pub raw_alloc: *c_void;
	pub raw_image: *c_ushort;
	pub color4_image: [*c_ushort; 4];
	pub color3_image: [*c_ushort; 3];
	pub ph1_cblack: [*c_short; 2];
	pub ph1_rblack: [*c_short; 2];
	pub iparams: libraw_iparams_t;
	pub sizes: libraw_image_sizes_t;
	pub ioparams: libraw_internal_output_params_t;
	pub color: libraw_colordata_t;
}

#[repr(C)]
pub struct libraw_makernotes_lens_t {
	pub LensID: c_ulong;
	pub Lens: [c_char; 128];
	pub LensFormat: c_ushort;
	pub LensMount: c_ushort;
	pub CamID: c_ulong;
	pub CameraFormat: c_ushort;
	pub CameraMount: c_ushort;
	pub body: [c_char; 64];
	pub FocalType: c_short;
	pub LensFeatures_pre: [c_char; 16];
	pub LensFeatures_suf: [c_char; 16];
	pub MinFocal: c_float;
	pub MaxFocal: c_float;
	pub MaxAp4MinFocal: c_float;
	pub MaxAp4MaxFocal: c_float;
	pub MinAp4MinFocal: c_float;
	pub MinAp4MaxFocal: c_float;
	pub MaxAp: c_float;
	pub MinAp: c_float;
	pub CurFocal: c_float;
	pub CurAp: c_float;
	pub MaxAp4CurFocal: c_float;
	pub MinAp4CurFocal: c_float;
	pub LensFStops: c_float;
	pub TeleconverterID: c_ulong;
	pub Teleconverter: [c_char; 128];
	pub AdapterID: c_ulong;
	pub Adapter: [c_char; 128];
	pub AttachmentID: c_ulong;
	pub Attachment: [c_char; 128];
	pub CanonFocalUnits: c_short;
	pub FocalLengthIn35mmFormat: c_float;
}

#[repr(C)]
pub struct libraw_nikonlens_t {
	pub NikonEffectiveMaxAp: c_float;
	pub NikonLensIDNumber: uchar;
	pub NikonLensFStops: uchar;
	pub NikonMCUVersion: uchar;
	pub NikonLensType: uchar;
}

#[repr(C)]
pub struct libraw_dnglens_t {
	pub MinFocal: c_float;
	pub MaxFocal: c_float;
	pub MaxAp4MinFocal: c_float;
	pub MaxAp4MaxFocal: c_float;
}

#[repr(C)]
pub struct libraw_lensinfo_t {
	pub MinFocal: c_float;
	pub MaxFocal: c_float;
	pub MaxAp4MinFocal: c_float;
	pub MaxAp4MaxFocal: c_float;
	pub EXIF_MaxAp: c_float;
	pub LensMake: [c_char; 128];
	pub Lens: [c_char; 128];
	pub FocalLengthIn35mmFormat: c_ushort;
	pub nikon: libraw_nikonlens_t;
	pub dng: libraw_dnglens_t;
	pub makernotes: libraw_makernotes_lens_t;
}

#[repr(C)]
pub struct libraw_data_t {
	pub image: [*c_ushort; 4];
	pub sizes: libraw_image_sizes_t;
	pub idata: libraw_iparams_t;
	pub lens: libraw_lensinfo_t;
	pub params: libraw_output_params_t;
	pub progress_flags: c_uint;
	pub process_warnings: c_uint;
	pub color: libraw_colordata_t;
	pub other: libraw_imgother_t;
	pub thumbnail: libraw_thumbnail_t;
	pub rawdata: libraw_rawdata_t;
	pub parent_class: *c_void;
}

