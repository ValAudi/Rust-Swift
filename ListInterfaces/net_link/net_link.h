#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct InterfaceData {
  uint32_t name_size;
  int8_t *name;
  uint32_t description_size;
  int8_t *description;
  uint32_t index;
  uint8_t mac[6];
  uint32_t ips_size;
  int8_t *ips;
  unsigned int flag;
} InterfaceData;

struct InterfaceData *get_interface_data(void);

uint32_t get_interface_data_count(void);
