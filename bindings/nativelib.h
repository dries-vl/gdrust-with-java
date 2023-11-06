#ifndef __NATIVELIB_H
#define __NATIVELIB_H

#include "graal_isolate.h"


#if defined(__cplusplus)
extern "C" {
#endif

void load_countries(graal_isolatethread_t*);

size_t get_country(graal_isolatethread_t*, int);

void update_country(graal_isolatethread_t*, int);

int run_main(int argc, char** argv);

#if defined(__cplusplus)
}
#endif
#endif
