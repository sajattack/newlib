#ifndef _ARPA_INET_H
#define _ARPA_INET_H

#include <sys/types.h>
#include <stdint.h>
#include <netinet/in.h>

uint32_t htonl(uint32_t hostlong);
uint16_t htons(uint16_t hostshort);
uint32_t ntohl(uint32_t netlong);
uint16_t ntohs(uint16_t netshort);

char *inet_ntoa(struct in_addr in);
int inet_aton(const char *cp, struct in_addr *inp);

#endif
