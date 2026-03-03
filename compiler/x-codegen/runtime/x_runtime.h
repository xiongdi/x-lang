/*
 * X-Lang C23 Runtime
 * Tagged-value runtime for dynamically typed X language programs.
 */
#ifndef X_RUNTIME_H
#define X_RUNTIME_H

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdint.h>
#include <string.h>
#include <math.h>
#include <ctype.h>

/* ========== Tagged Value ========== */

typedef enum {
    X_INT = 0,
    X_FLOAT,
    X_BOOL,
    X_STRING,
    X_ARRAY,
    X_MAP,
    X_NULL,
    X_NONE,
} XValueTag;

typedef struct XArray XArray;
typedef struct XMap XMap;
typedef struct XMapEntry XMapEntry;

typedef struct XValue {
    XValueTag tag;
    union {
        int64_t i;
        double f;
        bool b;
        char *s;
        XArray *arr;
        XMap *map;
    } as;
} XValue;

struct XArray {
    XValue *items;
    int64_t length;
    int64_t capacity;
    int64_t refcount;
};

struct XMapEntry {
    char *key;
    XValue value;
    struct XMapEntry *next;
};

struct XMap {
    XMapEntry **buckets;
    int64_t num_buckets;
    int64_t size;
    int64_t refcount;
};

/* ========== Value constructors ========== */

static inline XValue x_int(int64_t v) {
    XValue r; r.tag = X_INT; r.as.i = v; return r;
}

static inline XValue x_float(double v) {
    XValue r; r.tag = X_FLOAT; r.as.f = v; return r;
}

static inline XValue x_bool(bool v) {
    XValue r; r.tag = X_BOOL; r.as.b = v; return r;
}

static inline XValue x_null(void) {
    XValue r; r.tag = X_NULL; r.as.i = 0; return r;
}

static inline XValue x_none(void) {
    XValue r; r.tag = X_NONE; r.as.i = 0; return r;
}

static inline XValue x_string(const char *s) {
    XValue r;
    r.tag = X_STRING;
    r.as.s = (char *)malloc(strlen(s) + 1);
    strcpy(r.as.s, s);
    return r;
}

static inline XValue x_string_own(char *s) {
    XValue r; r.tag = X_STRING; r.as.s = s; return r;
}

/* ========== Array ========== */

static inline XArray *x_array_new(int64_t cap) {
    XArray *a = (XArray *)malloc(sizeof(XArray));
    a->length = 0;
    a->capacity = cap > 4 ? cap : 4;
    a->items = (XValue *)malloc(sizeof(XValue) * a->capacity);
    a->refcount = 1;
    return a;
}

static inline XValue x_array_val(XArray *a) {
    XValue r; r.tag = X_ARRAY; r.as.arr = a; return r;
}

static inline void x_array_push(XArray *a, XValue v) {
    if (a->length >= a->capacity) {
        a->capacity *= 2;
        a->items = (XValue *)realloc(a->items, sizeof(XValue) * a->capacity);
    }
    a->items[a->length++] = v;
}

static inline XValue x_array_get(XArray *a, int64_t idx) {
    if (idx < 0 || idx >= a->length) {
        fprintf(stderr, "Array index out of bounds: %lld (length %lld)\n",
                (long long)idx, (long long)a->length);
        exit(1);
    }
    return a->items[idx];
}

static inline void x_array_set(XArray *a, int64_t idx, XValue v) {
    if (idx < 0 || idx >= a->length) {
        fprintf(stderr, "Array index out of bounds: %lld (length %lld)\n",
                (long long)idx, (long long)a->length);
        exit(1);
    }
    a->items[idx] = v;
}

/* ========== Map ========== */

static inline uint64_t x_hash_str(const char *s) {
    uint64_t h = 5381;
    while (*s) { h = h * 33 + (unsigned char)*s; s++; }
    return h;
}

static inline XMap *x_map_new(void) {
    XMap *m = (XMap *)malloc(sizeof(XMap));
    m->num_buckets = 64;
    m->buckets = (XMapEntry **)calloc(m->num_buckets, sizeof(XMapEntry *));
    m->size = 0;
    m->refcount = 1;
    return m;
}

static inline XValue x_map_val(XMap *m) {
    XValue r; r.tag = X_MAP; r.as.map = m; return r;
}

static inline void x_map_set(XMap *m, const char *key, XValue val) {
    uint64_t h = x_hash_str(key) % m->num_buckets;
    XMapEntry *e = m->buckets[h];
    while (e) {
        if (strcmp(e->key, key) == 0) {
            e->value = val;
            return;
        }
        e = e->next;
    }
    XMapEntry *ne = (XMapEntry *)malloc(sizeof(XMapEntry));
    ne->key = (char *)malloc(strlen(key) + 1);
    strcpy(ne->key, key);
    ne->value = val;
    ne->next = m->buckets[h];
    m->buckets[h] = ne;
    m->size++;
}

static inline XValue x_map_get(XMap *m, const char *key) {
    uint64_t h = x_hash_str(key) % m->num_buckets;
    XMapEntry *e = m->buckets[h];
    while (e) {
        if (strcmp(e->key, key) == 0) return e->value;
        e = e->next;
    }
    return x_int(0);
}

static inline bool x_map_contains(XMap *m, const char *key) {
    uint64_t h = x_hash_str(key) % m->num_buckets;
    XMapEntry *e = m->buckets[h];
    while (e) {
        if (strcmp(e->key, key) == 0) return true;
        e = e->next;
    }
    return false;
}

static inline XValue x_map_keys(XMap *m) {
    XArray *keys = x_array_new(m->size);
    for (int64_t i = 0; i < m->num_buckets; i++) {
        XMapEntry *e = m->buckets[i];
        while (e) {
            x_array_push(keys, x_string(e->key));
            e = e->next;
        }
    }
    return x_array_val(keys);
}

/* ========== Type coercions ========== */

static inline double x_as_float(XValue v) {
    if (v.tag == X_FLOAT) return v.as.f;
    if (v.tag == X_INT) return (double)v.as.i;
    if (v.tag == X_BOOL) return v.as.b ? 1.0 : 0.0;
    return 0.0;
}

static inline int64_t x_as_int(XValue v) {
    if (v.tag == X_INT) return v.as.i;
    if (v.tag == X_FLOAT) return (int64_t)v.as.f;
    if (v.tag == X_BOOL) return v.as.b ? 1 : 0;
    return 0;
}

static inline bool x_as_bool(XValue v) {
    switch (v.tag) {
        case X_BOOL: return v.as.b;
        case X_INT: return v.as.i != 0;
        case X_FLOAT: return v.as.f != 0.0;
        case X_STRING: return v.as.s != NULL && v.as.s[0] != '\0';
        case X_NULL: case X_NONE: return false;
        default: return true;
    }
}

static inline const char *x_as_str(XValue v) {
    if (v.tag == X_STRING) return v.as.s;
    return "";
}

/* ========== Print ========== */

static inline void x_print(XValue v) {
    switch (v.tag) {
        case X_INT: printf("%lld\n", (long long)v.as.i); break;
        case X_FLOAT: printf("%g\n", v.as.f); break;
        case X_BOOL: printf("%s\n", v.as.b ? "true" : "false"); break;
        case X_STRING: printf("%s\n", v.as.s); break;
        case X_NULL: printf("null\n"); break;
        case X_NONE: printf("none\n"); break;
        case X_ARRAY: printf("[array len=%lld]\n", (long long)v.as.arr->length); break;
        case X_MAP: printf("[map size=%lld]\n", (long long)v.as.map->size); break;
    }
}

static inline void x_print_inline(XValue v) {
    switch (v.tag) {
        case X_INT: printf("%lld", (long long)v.as.i); break;
        case X_FLOAT: printf("%g", v.as.f); break;
        case X_BOOL: printf("%s", v.as.b ? "true" : "false"); break;
        case X_STRING: printf("%s", v.as.s); break;
        default: break;
    }
}

/* ========== String operations ========== */

static inline XValue x_concat(XValue a, XValue b) {
    const char *sa = x_as_str(a);
    const char *sb = x_as_str(b);
    size_t la = strlen(sa), lb = strlen(sb);
    char *buf = (char *)malloc(la + lb + 1);
    memcpy(buf, sa, la);
    memcpy(buf + la, sb, lb);
    buf[la + lb] = '\0';
    return x_string_own(buf);
}

static inline XValue x_to_string(XValue v) {
    char buf[64];
    switch (v.tag) {
        case X_INT: snprintf(buf, sizeof(buf), "%lld", (long long)v.as.i); return x_string(buf);
        case X_FLOAT: snprintf(buf, sizeof(buf), "%g", v.as.f); return x_string(buf);
        case X_BOOL: return x_string(v.as.b ? "true" : "false");
        case X_STRING: return v;
        case X_NULL: return x_string("null");
        case X_NONE: return x_string("none");
        default: return x_string("[object]");
    }
}

static inline XValue x_char_at(XValue s, XValue idx) {
    const char *str = x_as_str(s);
    int64_t i = x_as_int(idx);
    int64_t slen = (int64_t)strlen(str);
    if (i < 0 || i >= slen) return x_string("");
    char buf[2] = { str[i], '\0' };
    return x_string(buf);
}

static inline XValue x_substring(XValue s, XValue start, XValue end) {
    const char *str = x_as_str(s);
    int64_t slen = (int64_t)strlen(str);
    int64_t st = x_as_int(start);
    int64_t en = x_as_int(end);
    if (st < 0) st = 0;
    if (en > slen) en = slen;
    if (st >= en) return x_string("");
    int64_t rlen = en - st;
    char *buf = (char *)malloc(rlen + 1);
    memcpy(buf, str + st, rlen);
    buf[rlen] = '\0';
    return x_string_own(buf);
}

static inline XValue x_len(XValue v) {
    if (v.tag == X_STRING) return x_int((int64_t)strlen(v.as.s));
    if (v.tag == X_ARRAY) return x_int(v.as.arr->length);
    if (v.tag == X_MAP) return x_int(v.as.map->size);
    return x_int(0);
}

static inline XValue x_str_upper(XValue v) {
    const char *s = x_as_str(v);
    size_t n = strlen(s);
    char *buf = (char *)malloc(n + 1);
    for (size_t i = 0; i < n; i++) buf[i] = toupper((unsigned char)s[i]);
    buf[n] = '\0';
    return x_string_own(buf);
}

static inline XValue x_str_lower(XValue v) {
    const char *s = x_as_str(v);
    size_t n = strlen(s);
    char *buf = (char *)malloc(n + 1);
    for (size_t i = 0; i < n; i++) buf[i] = tolower((unsigned char)s[i]);
    buf[n] = '\0';
    return x_string_own(buf);
}

static inline XValue x_str_trim(XValue v) {
    const char *s = x_as_str(v);
    while (*s && isspace((unsigned char)*s)) s++;
    const char *end = s + strlen(s) - 1;
    while (end > s && isspace((unsigned char)*end)) end--;
    size_t n = end - s + 1;
    char *buf = (char *)malloc(n + 1);
    memcpy(buf, s, n);
    buf[n] = '\0';
    return x_string_own(buf);
}

static inline XValue x_str_split(XValue str_v, XValue delim_v) {
    const char *s = x_as_str(str_v);
    const char *d = x_as_str(delim_v);
    size_t dlen = strlen(d);
    XArray *arr = x_array_new(8);
    if (dlen == 0) {
        x_array_push(arr, x_string(s));
        return x_array_val(arr);
    }
    const char *pos = s;
    const char *found;
    while ((found = strstr(pos, d)) != NULL) {
        size_t seg = found - pos;
        char *buf = (char *)malloc(seg + 1);
        memcpy(buf, pos, seg);
        buf[seg] = '\0';
        x_array_push(arr, x_string_own(buf));
        pos = found + dlen;
    }
    x_array_push(arr, x_string(pos));
    return x_array_val(arr);
}

static inline XValue x_str_starts_with(XValue str_v, XValue prefix_v) {
    const char *s = x_as_str(str_v);
    const char *p = x_as_str(prefix_v);
    return x_bool(strncmp(s, p, strlen(p)) == 0);
}

static inline XValue x_str_contains(XValue str_v, XValue sub_v) {
    return x_bool(strstr(x_as_str(str_v), x_as_str(sub_v)) != NULL);
}

static inline XValue x_str_find(XValue str_v, XValue sub_v) {
    const char *s = x_as_str(str_v);
    const char *sub = x_as_str(sub_v);
    const char *p = strstr(s, sub);
    if (p) return x_int((int64_t)(p - s));
    return x_int(-1);
}

static inline XValue x_str_replace(XValue str_v, XValue old_v, XValue new_v) {
    const char *s = x_as_str(str_v);
    const char *old_s = x_as_str(old_v);
    const char *new_s = x_as_str(new_v);
    size_t old_len = strlen(old_s);
    size_t new_len = strlen(new_s);
    if (old_len == 0) return str_v;
    size_t count = 0;
    const char *p = s;
    while ((p = strstr(p, old_s)) != NULL) { count++; p += old_len; }
    size_t result_len = strlen(s) + count * (new_len - old_len);
    char *buf = (char *)malloc(result_len + 1);
    char *out = buf;
    p = s;
    const char *found;
    while ((found = strstr(p, old_s)) != NULL) {
        size_t seg = found - p;
        memcpy(out, p, seg); out += seg;
        memcpy(out, new_s, new_len); out += new_len;
        p = found + old_len;
    }
    strcpy(out, p);
    return x_string_own(buf);
}

/* ========== Array builtins ========== */

static inline XValue x_new_array(XValue size, XValue init) {
    int64_t n = x_as_int(size);
    XArray *a = x_array_new(n);
    for (int64_t i = 0; i < n; i++) {
        x_array_push(a, init);
    }
    return x_array_val(a);
}

static inline XValue x_push(XValue arr_v, XValue val) {
    if (arr_v.tag != X_ARRAY) return arr_v;
    x_array_push(arr_v.as.arr, val);
    return arr_v;
}

static inline XValue x_copy_array(XValue arr_v) {
    if (arr_v.tag != X_ARRAY) return arr_v;
    XArray *src = arr_v.as.arr;
    XArray *dst = x_array_new(src->length);
    for (int64_t i = 0; i < src->length; i++) {
        x_array_push(dst, src->items[i]);
    }
    return x_array_val(dst);
}

static inline XValue x_swap(XValue arr_v, XValue i_v, XValue j_v) {
    if (arr_v.tag != X_ARRAY) return x_null();
    int64_t i = x_as_int(i_v);
    int64_t j = x_as_int(j_v);
    XArray *a = arr_v.as.arr;
    XValue tmp = a->items[i];
    a->items[i] = a->items[j];
    a->items[j] = tmp;
    return x_null();
}

static int x_sort_cmp_desc(const void *a, const void *b) {
    const XValue *va = (const XValue *)a;
    const XValue *vb = (const XValue *)b;
    if (va->tag != X_ARRAY || vb->tag != X_ARRAY) return 0;
    XValue ca = va->as.arr->items[1];
    XValue cb = vb->as.arr->items[1];
    double da = x_as_float(ca), db = x_as_float(cb);
    if (db > da) return 1;
    if (db < da) return -1;
    return 0;
}

static inline XValue x_sort_by_value_desc(XValue arr_v) {
    if (arr_v.tag != X_ARRAY) return x_null();
    XArray *a = arr_v.as.arr;
    qsort(a->items, a->length, sizeof(XValue), x_sort_cmp_desc);
    return x_null();
}

/* ========== Math builtins ========== */

static inline XValue x_sqrt(XValue v) { return x_float(sqrt(x_as_float(v))); }
static inline XValue x_pow(XValue base, XValue exp) { return x_float(pow(x_as_float(base), x_as_float(exp))); }
static inline XValue x_abs(XValue v) { return x_float(fabs(x_as_float(v))); }
static inline XValue x_sin(XValue v) { return x_float(sin(x_as_float(v))); }
static inline XValue x_cos(XValue v) { return x_float(cos(x_as_float(v))); }
static inline XValue x_floor(XValue v) { return x_float(floor(x_as_float(v))); }
static inline XValue x_ceil(XValue v) { return x_float(ceil(x_as_float(v))); }
static inline XValue x_round(XValue v) { return x_float(round(x_as_float(v))); }

/* ========== Type conversions ========== */

static inline XValue x_to_int(XValue v) { return x_int(x_as_int(v)); }
static inline XValue x_to_float(XValue v) { return x_float(x_as_float(v)); }

static inline XValue x_format_float(XValue v, XValue prec) {
    char buf[64];
    snprintf(buf, sizeof(buf), "%.*f", (int)x_as_int(prec), x_as_float(v));
    return x_string(buf);
}

static inline XValue x_type_of(XValue v) {
    switch (v.tag) {
        case X_INT: return x_string("int");
        case X_FLOAT: return x_string("float");
        case X_BOOL: return x_string("bool");
        case X_STRING: return x_string("string");
        case X_ARRAY: return x_string("array");
        case X_MAP: return x_string("map");
        case X_NULL: return x_string("null");
        case X_NONE: return x_string("none");
    }
    return x_string("unknown");
}

/* ========== Arithmetic on XValue ========== */

static inline XValue x_add(XValue a, XValue b) {
    if (a.tag == X_STRING || b.tag == X_STRING) return x_concat(a.tag == X_STRING ? a : x_to_string(a), b.tag == X_STRING ? b : x_to_string(b));
    if (a.tag == X_FLOAT || b.tag == X_FLOAT) return x_float(x_as_float(a) + x_as_float(b));
    return x_int(a.as.i + b.as.i);
}

static inline XValue x_sub(XValue a, XValue b) {
    if (a.tag == X_FLOAT || b.tag == X_FLOAT) return x_float(x_as_float(a) - x_as_float(b));
    return x_int(a.as.i - b.as.i);
}

static inline XValue x_mul(XValue a, XValue b) {
    if (a.tag == X_FLOAT || b.tag == X_FLOAT) return x_float(x_as_float(a) * x_as_float(b));
    return x_int(a.as.i * b.as.i);
}

static inline XValue x_div(XValue a, XValue b) {
    if (a.tag == X_FLOAT || b.tag == X_FLOAT) return x_float(x_as_float(a) / x_as_float(b));
    int64_t bv = b.as.i;
    if (bv == 0) { fprintf(stderr, "Division by zero\n"); exit(1); }
    return x_int(a.as.i / bv);
}

static inline XValue x_mod(XValue a, XValue b) {
    if (a.tag == X_FLOAT || b.tag == X_FLOAT) return x_float(fmod(x_as_float(a), x_as_float(b)));
    int64_t bv = b.as.i;
    if (bv == 0) { fprintf(stderr, "Modulo by zero\n"); exit(1); }
    return x_int(a.as.i % bv);
}

static inline XValue x_neg(XValue a) {
    if (a.tag == X_FLOAT) return x_float(-a.as.f);
    return x_int(-a.as.i);
}

static inline XValue x_not(XValue a) { return x_bool(!x_as_bool(a)); }

/* ========== Comparison ========== */

static inline XValue x_eq(XValue a, XValue b) {
    if (a.tag == X_STRING && b.tag == X_STRING) return x_bool(strcmp(a.as.s, b.as.s) == 0);
    if (a.tag == X_INT && b.tag == X_INT) return x_bool(a.as.i == b.as.i);
    if (a.tag == X_FLOAT || b.tag == X_FLOAT) return x_bool(x_as_float(a) == x_as_float(b));
    if (a.tag == X_BOOL && b.tag == X_BOOL) return x_bool(a.as.b == b.as.b);
    if (a.tag == X_NULL && b.tag == X_NULL) return x_bool(true);
    if (a.tag == X_NONE && b.tag == X_NONE) return x_bool(true);
    return x_bool(false);
}

static inline XValue x_neq(XValue a, XValue b) { return x_bool(!x_as_bool(x_eq(a, b))); }

static inline XValue x_lt(XValue a, XValue b) {
    if (a.tag == X_FLOAT || b.tag == X_FLOAT) return x_bool(x_as_float(a) < x_as_float(b));
    return x_bool(a.as.i < b.as.i);
}

static inline XValue x_le(XValue a, XValue b) {
    if (a.tag == X_FLOAT || b.tag == X_FLOAT) return x_bool(x_as_float(a) <= x_as_float(b));
    return x_bool(a.as.i <= b.as.i);
}

static inline XValue x_gt(XValue a, XValue b) {
    if (a.tag == X_FLOAT || b.tag == X_FLOAT) return x_bool(x_as_float(a) > x_as_float(b));
    return x_bool(a.as.i > b.as.i);
}

static inline XValue x_ge(XValue a, XValue b) {
    if (a.tag == X_FLOAT || b.tag == X_FLOAT) return x_bool(x_as_float(a) >= x_as_float(b));
    return x_bool(a.as.i >= b.as.i);
}

static inline XValue x_and(XValue a, XValue b) { return x_bool(x_as_bool(a) && x_as_bool(b)); }
static inline XValue x_or(XValue a, XValue b) { return x_bool(x_as_bool(a) || x_as_bool(b)); }

/* ========== Index operations ========== */

static inline XValue x_index_get(XValue obj, XValue idx) {
    if (obj.tag == X_ARRAY) return x_array_get(obj.as.arr, x_as_int(idx));
    if (obj.tag == X_STRING) return x_char_at(obj, idx);
    if (obj.tag == X_MAP && idx.tag == X_STRING) return x_map_get(obj.as.map, idx.as.s);
    fprintf(stderr, "Cannot index value of type %d\n", obj.tag);
    exit(1);
}

static inline void x_index_set(XValue obj, XValue idx, XValue val) {
    if (obj.tag == X_ARRAY) { x_array_set(obj.as.arr, x_as_int(idx), val); return; }
    if (obj.tag == X_MAP && idx.tag == X_STRING) { x_map_set(obj.as.map, idx.as.s, val); return; }
    fprintf(stderr, "Cannot set index on value of type %d\n", obj.tag);
    exit(1);
}

/* ========== Regex (simple pattern matcher) ========== */

static int x_simple_match_at(const char *text, int tpos, int tlen,
                              const char *pat, int ppos, int plen);

static int x_simple_match_at(const char *text, int tpos, int tlen,
                              const char *pat, int ppos, int plen) {
    while (ppos < plen && tpos <= tlen) {
        if (pat[ppos] == '[') {
            int end = ppos + 1;
            while (end < plen && pat[end] != ']') end++;
            if (tpos >= tlen) return 0;
            bool found = false;
            for (int k = ppos + 1; k < end; k++) {
                if (pat[k] == text[tpos]) { found = true; break; }
            }
            if (!found) return 0;
            ppos = end + 1;
            tpos++;
        } else if (pat[ppos] == '.') {
            if (tpos >= tlen) return 0;
            ppos++; tpos++;
        } else {
            if (tpos >= tlen || pat[ppos] != text[tpos]) return 0;
            ppos++; tpos++;
        }
    }
    return ppos >= plen ? 1 : 0;
}

static inline int64_t x_regex_count_branch(const char *text, int tlen,
                                            const char *pat, int plen) {
    int64_t count = 0;
    for (int i = 0; i <= tlen - plen + 10; i++) {
        if (i >= tlen) break;
        if (x_simple_match_at(text, i, tlen, pat, 0, plen)) count++;
    }
    return count;
}

static inline XValue x_regex_match_count(XValue text_v, XValue pattern_v) {
    const char *text = x_as_str(text_v);
    const char *pat = x_as_str(pattern_v);
    int tlen = (int)strlen(text);

    int64_t total = 0;
    const char *p = pat;
    while (*p) {
        const char *bar = strchr(p, '|');
        int blen;
        if (bar) {
            blen = (int)(bar - p);
        } else {
            blen = (int)strlen(p);
        }
        char *branch = (char *)malloc(blen + 1);
        memcpy(branch, p, blen);
        branch[blen] = '\0';
        total += x_regex_count_branch(text, tlen, branch, blen);
        free(branch);
        if (bar) p = bar + 1; else break;
    }
    return x_int(total);
}

/* ========== Pi digits (Gibbons' spigot with simple bignum) ========== */

/* Simple arbitrary-precision integer for pi computation */
typedef struct { uint32_t *d; int len; int cap; int sign; } xbn;

static xbn xbn_new(int cap) { xbn b; b.cap=cap<4?4:cap; b.len=0; b.sign=1; b.d=(uint32_t*)calloc(b.cap,sizeof(uint32_t)); return b; }
static void xbn_free(xbn *b) { free(b->d); b->d=NULL; b->len=0; }
static void xbn_ensure(xbn *b, int n) { if(n>b->cap){b->cap=n*2;b->d=(uint32_t*)realloc(b->d,b->cap*sizeof(uint32_t));} }
static xbn xbn_from(int64_t v) {
    xbn b=xbn_new(4);
    if(v<0){b.sign=-1;v=-v;}
    if(v==0){b.len=1;b.d[0]=0;return b;}
    while(v>0){xbn_ensure(&b,b.len+1);b.d[b.len++]=(uint32_t)(v&0xFFFFFFFF);v>>=32;}
    return b;
}
static xbn xbn_copy(xbn *a) { xbn b=xbn_new(a->len); b.len=a->len; b.sign=a->sign; memcpy(b.d,a->d,a->len*sizeof(uint32_t)); return b; }

static xbn xbn_add_abs(xbn *a, xbn *b) {
    int n=a->len>b->len?a->len:b->len;
    xbn r=xbn_new(n+1); r.len=n+1;
    uint64_t carry=0;
    for(int i=0;i<n+1;i++){
        uint64_t va=i<a->len?(uint64_t)a->d[i]:0;
        uint64_t vb=i<b->len?(uint64_t)b->d[i]:0;
        uint64_t s=va+vb+carry; r.d[i]=(uint32_t)(s&0xFFFFFFFF); carry=s>>32;
    }
    while(r.len>1&&r.d[r.len-1]==0)r.len--;
    return r;
}

static int xbn_cmp_abs(xbn *a, xbn *b) {
    if(a->len!=b->len) return a->len>b->len?1:-1;
    for(int i=a->len-1;i>=0;i--) { if(a->d[i]!=b->d[i]) return a->d[i]>b->d[i]?1:-1; }
    return 0;
}

static xbn xbn_sub_abs(xbn *a, xbn *b) {
    xbn r=xbn_new(a->len); r.len=a->len;
    int64_t borrow=0;
    for(int i=0;i<a->len;i++){
        int64_t va=(int64_t)a->d[i];
        int64_t vb=i<b->len?(int64_t)b->d[i]:0;
        int64_t d=va-vb-borrow;
        if(d<0){d+=(int64_t)1<<32;borrow=1;}else{borrow=0;}
        r.d[i]=(uint32_t)d;
    }
    while(r.len>1&&r.d[r.len-1]==0)r.len--;
    return r;
}

static xbn xbn_mul_small(xbn *a, uint64_t v) {
    xbn r=xbn_new(a->len+2); r.len=a->len+2; r.sign=a->sign;
    uint64_t carry=0;
    for(int i=0;i<a->len;i++){
        uint64_t p=(uint64_t)a->d[i]*v+carry;
        r.d[i]=(uint32_t)(p&0xFFFFFFFF); carry=p>>32;
    }
    r.d[a->len]=(uint32_t)(carry&0xFFFFFFFF);
    r.d[a->len+1]=(uint32_t)(carry>>32);
    while(r.len>1&&r.d[r.len-1]==0)r.len--;
    return r;
}

static xbn xbn_add(xbn *a, xbn *b) {
    if(a->sign==b->sign){ xbn r=xbn_add_abs(a,b); r.sign=a->sign; return r; }
    int c=xbn_cmp_abs(a,b);
    if(c==0) return xbn_from(0);
    if(c>0){ xbn r=xbn_sub_abs(a,b); r.sign=a->sign; return r; }
    xbn r=xbn_sub_abs(b,a); r.sign=b->sign; return r;
}

static xbn xbn_mul(xbn *a, xbn *b) {
    int n=a->len+b->len;
    xbn r=xbn_new(n); r.len=n; r.sign=a->sign*b->sign;
    for(int i=0;i<a->len;i++){
        uint64_t carry=0;
        for(int j=0;j<b->len;j++){
            uint64_t p=(uint64_t)a->d[i]*(uint64_t)b->d[j]+(uint64_t)r.d[i+j]+carry;
            r.d[i+j]=(uint32_t)(p&0xFFFFFFFF); carry=p>>32;
        }
        r.d[i+b->len]+=(uint32_t)carry;
    }
    while(r.len>1&&r.d[r.len-1]==0)r.len--;
    return r;
}

static xbn xbn_div(xbn *num, xbn *den, xbn *rem) {
    if(den->len==1&&den->d[0]==0){*rem=xbn_from(0);return xbn_from(0);}
    if(xbn_cmp_abs(num,den)<0){if(rem)*rem=xbn_copy(num);return xbn_from(0);}
    if(den->len==1){
        uint64_t dv=den->d[0]; uint64_t carry=0;
        xbn q=xbn_new(num->len); q.len=num->len; q.sign=num->sign*den->sign;
        for(int i=num->len-1;i>=0;i--){
            uint64_t cur=carry*(((uint64_t)1)<<32)+(uint64_t)num->d[i];
            q.d[i]=(uint32_t)(cur/dv); carry=cur%dv;
        }
        while(q.len>1&&q.d[q.len-1]==0)q.len--;
        if(rem)*rem=xbn_from((int64_t)carry);
        return q;
    }
    /* For multi-word division, use repeated subtraction (slow but correct for small numbers) */
    xbn a=xbn_copy(num); a.sign=1;
    xbn b=xbn_copy(den); b.sign=1;
    xbn q=xbn_from(0);
    while(xbn_cmp_abs(&a,&b)>=0){
        xbn t=xbn_sub_abs(&a,&b);
        xbn_free(&a); a=t;
        xbn one=xbn_from(1);
        xbn nq=xbn_add(&q,&one);
        xbn_free(&q); q=nq;
        xbn_free(&one);
    }
    q.sign=num->sign*den->sign;
    if(rem){*rem=a;}else{xbn_free(&a);}
    xbn_free(&b);
    return q;
}

static inline XValue x_compute_pi_digits(XValue n_v) {
    int n = (int)x_as_int(n_v);
    char *result = (char *)malloc(n + 2);
    int result_len = 0;

    xbn q=xbn_from(1), r=xbn_from(0), t=xbn_from(1);
    int64_t k=0;

    while (result_len < n) {
        k++;
        int64_t l = 2*k+1;
        /* compose: q,r,t = q*k, (2*q+r)*l, t*l */
        xbn qk=xbn_mul_small(&q,(uint64_t)k);
        xbn twoq=xbn_mul_small(&q,2);
        xbn twoq_r=xbn_add(&twoq,&r);
        xbn nr=xbn_mul_small(&twoq_r,(uint64_t)l);
        xbn nt=xbn_mul_small(&t,(uint64_t)l);
        xbn_free(&twoq); xbn_free(&twoq_r);
        xbn_free(&q); xbn_free(&r); xbn_free(&t);
        q=qk; r=nr; t=nt;

        /* extract: digit = (3*q+r)/t vs (4*q+r)/t */
        xbn q3=xbn_mul_small(&q,3);
        xbn q3r=xbn_add(&q3,&r);
        xbn rem1;
        xbn d1=xbn_div(&q3r,&t,&rem1);

        xbn q4=xbn_mul_small(&q,4);
        xbn q4r=xbn_add(&q4,&r);
        xbn rem2;
        xbn d2=xbn_div(&q4r,&t,&rem2);

        int64_t digit1 = (d1.len==1) ? (int64_t)d1.d[0]*d1.sign : -1;
        int64_t digit2 = (d2.len==1) ? (int64_t)d2.d[0]*d2.sign : -1;

        xbn_free(&q3); xbn_free(&q3r); xbn_free(&rem1);
        xbn_free(&q4); xbn_free(&q4r); xbn_free(&rem2);

        if (digit1 == digit2 && digit1 >= 0 && digit1 <= 9) {
            result[result_len++] = '0' + (int)digit1;
            /* extract: q,r,t = 10*q, 10*(r - digit*t), t */
            xbn dt=xbn_mul_small(&t,(uint64_t)digit1);
            dt.sign = -dt.sign;
            xbn rdiff=xbn_add(&r,&dt);
            xbn new_r=xbn_mul_small(&rdiff,10);
            xbn new_q=xbn_mul_small(&q,10);
            xbn_free(&dt); xbn_free(&rdiff);
            xbn_free(&q); xbn_free(&r);
            q=new_q; r=new_r;
        }
        xbn_free(&d1); xbn_free(&d2);
    }
    xbn_free(&q); xbn_free(&r); xbn_free(&t);
    result[result_len] = '\0';
    return x_string_own(result);
}

/* ========== Map builtins (wrapped) ========== */

static inline XValue x_builtin_new_map(void) { return x_map_val(x_map_new()); }

static inline XValue x_builtin_map_set(XValue map_v, XValue key_v, XValue val) {
    if (map_v.tag != X_MAP) return x_null();
    x_map_set(map_v.as.map, x_as_str(key_v), val);
    return x_null();
}

static inline XValue x_builtin_map_get(XValue map_v, XValue key_v) {
    if (map_v.tag != X_MAP) return x_int(0);
    return x_map_get(map_v.as.map, x_as_str(key_v));
}

static inline XValue x_builtin_map_contains(XValue map_v, XValue key_v) {
    if (map_v.tag != X_MAP) return x_bool(false);
    return x_bool(x_map_contains(map_v.as.map, x_as_str(key_v)));
}

static inline XValue x_builtin_map_keys(XValue map_v) {
    if (map_v.tag != X_MAP) return x_array_val(x_array_new(0));
    return x_map_keys(map_v.as.map);
}

#endif /* X_RUNTIME_H */
