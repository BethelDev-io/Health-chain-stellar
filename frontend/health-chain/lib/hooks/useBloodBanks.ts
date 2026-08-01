/**
 * useBloodBanks – React Query hook for blood bank availability.
 *
 * Only fetches when enabled (step 2 active + blood type selected).
 * refetchInterval drives the 30-second polling fallback.
 */

import { useQuery } from "@tanstack/react-query";
import { queryKeys } from "@/lib/api/queryKeys";
import { fetchBloodBankAvailability } from "@/lib/api/bloodBanks.api";
import type { BloodType } from "@/lib/types/orders";

const POLLING_INTERVAL_MS = 30_000;

interface UseBloodBanksOptions {
  bloodType: BloodType | null;
  hospitalId: string;
  enabled?: boolean;
}

export function useBloodBanks({
  bloodType,
  hospitalId,
  enabled = true,
}: UseBloodBanksOptions) {
  return useQuery({
    queryKey: queryKeys.bloodBanks.availability(bloodType ?? "", hospitalId),
    queryFn: () => fetchBloodBankAvailability(bloodType!, hospitalId),
    enabled: enabled && bloodType !== null,
    refetchInterval: POLLING_INTERVAL_MS,
    staleTime: 15_000,
  });
}
