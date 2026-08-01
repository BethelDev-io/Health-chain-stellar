/**
 * Typed API functions for Orders.
 * All functions use the shared http-client (handles auth + token refresh).
 * Used exclusively as React Query queryFns / mutationFns – never called directly in components.
 */

import { api } from "./http-client";
import type {
  Order,
  OrdersResponse,
  OrderQueryParams,
  NewOrderPayload,
  NewOrderResponse,
} from "@/lib/types/orders";

const API_PREFIX = process.env.NEXT_PUBLIC_API_PREFIX || "api/v1";

// ─── Helpers ─────────────────────────────────────────────────────────────────

/** Parse serialised date strings back into Date objects */
function hydrateOrderDates(order: Order): Order {
  return {
    ...order,
    placedAt: new Date(order.placedAt),
    deliveredAt: order.deliveredAt ? new Date(order.deliveredAt) : null,
    confirmedAt: order.confirmedAt ? new Date(order.confirmedAt) : null,
    cancelledAt: order.cancelledAt ? new Date(order.cancelledAt) : null,
    createdAt: new Date(order.createdAt),
    updatedAt: new Date(order.updatedAt),
  };
}

// ─── API Functions ────────────────────────────────────────────────────────────

/**
 * Fetch paginated, filtered, sorted order list.
 * Maps directly to GET /api/v1/orders
 */
export async function fetchOrders(
  params: OrderQueryParams,
): Promise<OrdersResponse> {
  const query = new URLSearchParams();

  query.set("hospitalId", params.hospitalId);
  if (params.startDate)
    query.set("startDate", params.startDate.toISOString().split("T")[0]);
  if (params.endDate)
    query.set("endDate", params.endDate.toISOString().split("T")[0]);
  params.bloodTypes?.forEach((bt) => query.append("bloodType", bt));
  params.statuses?.forEach((s) => query.append("status", s));
  if (params.bloodBank) query.set("bloodBank", params.bloodBank);
  query.set("sortBy", params.column);
  query.set("sortOrder", params.order);
  query.set("page", String(params.page));
  query.set("pageSize", String(params.pageSize));

  const data = await api.get<OrdersResponse>(
    `/${API_PREFIX}/orders?${query.toString()}`,
  );

  return {
    ...data,
    data: data.data.map(hydrateOrderDates),
  };
}

/**
 * Place a new blood order.
 * Maps to POST /api/v1/orders
 */
export async function placeOrder(
  payload: NewOrderPayload,
): Promise<NewOrderResponse> {
  return api.post<NewOrderResponse>(`/${API_PREFIX}/orders`, payload);
}
