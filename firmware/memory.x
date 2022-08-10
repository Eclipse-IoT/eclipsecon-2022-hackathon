MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH                             : ORIGIN = 0x00027000, LENGTH = 360448
  STORAGE                           : ORIGIN = 0x0007F000, LENGTH = 4K
  RAM                               : ORIGIN = 0x2000CD28, LENGTH = 78552
}

__storage = ORIGIN(STORAGE);
