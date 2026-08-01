"use client";

/**
 * ReactQueryProvider
 *
 * Wraps the app in TanStack React Query's QueryClientProvider.
 * Must be a Client Component because QueryClient is stateful on the client.
 *
 * Devtools are included only when NODE_ENV !== 'production'.
 */

import React, { useState } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";

/** Shared QueryClient defaults */
function makeQueryClient() {
  return new QueryClient({
    defaultOptions: {
      queries: {
        // Cache stays fresh for 30 seconds before a background refetch
        staleTime: 30_000,
        // Keep unused data in cache for 5 minutes
        gcTime: 5 * 60 * 1000,
        // Retry once on failure (network blips)
        retry: 1,
        // Refetch when the browser tab regains focus
        refetchOnWindowFocus: true,
      },
      mutations: {
        // Retry mutations once on network error
        retry: 1,
      },
    },
  });
}

interface ReactQueryProviderProps {
  children: React.ReactNode;
}

export function ReactQueryProvider({ children }: ReactQueryProviderProps) {
  // Use useState to ensure a single QueryClient instance per component lifecycle.
  // This prevents server/client state sharing in RSC environments.
  const [queryClient] = useState(() => makeQueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      {children}
      {process.env.NODE_ENV !== "production" && (
        <ReactQueryDevtools initialIsOpen={false} />
      )}
    </QueryClientProvider>
  );
}
