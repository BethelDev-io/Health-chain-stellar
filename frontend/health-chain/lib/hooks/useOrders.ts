/**
 * useOrders – React Query hook for orders list.
 *
 * Encapsulates querying and cache invalidation so pages stay slim.
 * Consumes fetchOrders from the typed API layer.
 */

import { useQuery } from "@tanstack/react-query";
import { queryKeys } from "@/lib/api/queryKeys";
import { fetchOrders } from "@/lib/api/orders.api";
import type { OrderQueryParams } from "@/lib/types/orders";

export function useOrders(params: OrderQueryParams) {
  return useQuery({
    queryKey: queryKeys.orders.list(params),
    queryFn: () => fetchOrders(params),
    // Keep previous data visible while new data loads (avoids layout flash).
    placeholderData: (prev) => prev,
  });
}
