#include <stdio.h>

unsigned long
hash(unsigned char *str)
{
    unsigned long hash = 5381;
    int c;
    while (c = *str++)
        hash = ((hash << 5) + hash) + c; /* hash * 33 + c */
    return hash;
}


void main() {
    // with tags sorted
    unsigned long h = hash("dgpnfnxw.qxufgnlwp,gugdtstao=oxbodp,hggfvghceyfz=lnelpjdhpqj,sesdaofncycmbum=eodzjc");
    printf("Hash value: %lu\n", h);
}