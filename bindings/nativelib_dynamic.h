#ifndef __NATIVELIB_H
#define __NATIVELIB_H

#include <graal_isolate_dynamic.h>


#if defined(__cplusplus)
extern "C" {
#endif

typedef void (*load_countries_fn_t)(graal_isolatethread_t*);

typedef size_t (*get_country_fn_t)(graal_isolatethread_t*, int);

typedef void (*update_country_fn_t)(graal_isolatethread_t*, int);

typedef int (*run_main_fn_t)(int argc, char** argv);

#if defined(__cplusplus)
}
#endif
#endif
