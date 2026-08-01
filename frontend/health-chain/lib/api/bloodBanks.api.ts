/**
 * Typed API functions for Blood Bank availability.
 * Falls back to mock data when the backend endpoint is not yet available.
 */

import { api } from "./http-client";
import type { BloodType, BloodBankAvailability } from "@/lib/types/orders";

const API_PREFIX = process.env.NEXT_PUBLIC_API_PREFIX || "api/v1";

// ─── Mock fallback (remove once real endpoint is live) ────────────────────────

function generateMockBloodBanks(bloodType: BloodType): BloodBankAvailability[] {
  return [
    {
      id: "BB-001",
      name: "Central Blood Bank",
      location: "Lagos Island",
      latitude: 6.4541,
      longitude: 3.3947,
      distanceKm: 8.2,
      estimatedDeliveryMinutes: 25,
      stock: {
        "A+": 12,
        "A-": 3,
        "B+": 8,
        "B-": 0,
        "AB+": 5,
        "AB-": 2,
        "O+": 20,
        "O-": 1,
      },
      stockLevel: "adequate",
    },
    {
      id: "BB-002",
      name: "Mainland Blood Centre",
      location: "Yaba",
      latitude: 6.5095,
      longitude: 3.3711,
      distanceKm: 4.1,
      estimatedDeliveryMinutes: 15,
      stock: {
        "A+": 2,
        "A-": 0,
        "B+": 1,
        "B-": 0,
        "AB+": 0,
        "AB-": 0,
        "O+": 3,
        "O-": 0,
      },
      stockLevel: "critical",
    },
    {
      id: "BB-003",
      name: "Ikeja General Blood Store",
      location: "Ikeja",
      latitude: 6.6018,
      longitude: 3.3515,
      distanceKm: 9.8,
      estimatedDeliveryMinutes: 35,
      stock: {
        "A+": 0,
        "A-": 0,
        "B+": 0,
        "B-": 0,
        "AB+": 0,
        "AB-": 0,
        "O+": 0,
        "O-": 0,
      },
      stockLevel: "out_of_stock",
    },
    {
      id: "BB-004",
      name: "Victoria Island Medical Bank",
      location: "Victoria Island",
      latitude: 6.4281,
      longitude: 3.4219,
      distanceKm: 12.5,
      estimatedDeliveryMinutes: 40,
      stock: {
        "A+": 6,
        "A-": 4,
        "B+": 9,
        "B-": 3,
        "AB+": 2,
        "AB-": 1,
        "O+": 15,
        "O-": 5,
      },
      stockLevel: bloodType === "O-" ? "low" : "adequate",
    },
  ];
}

// ─── API Function ─────────────────────────────────────────────────────────────

/**
 * Fetch blood bank availability sorted by distance.
 * Falls back to mock data if the backend endpoint is not yet available.
 * Maps to GET /api/v1/blood-banks/availability
 */
export async function fetchBloodBankAvailability(
  bloodType: BloodType,
  hospitalId: string,
): Promise<BloodBankAvailability[]> {
  try {
    const data = await api.get<BloodBankAvailability[]>(
      `/${API_PREFIX}/blood-banks/availability?bloodType=${bloodType}&hospitalId=${hospitalId}`,
    );
    return [...data].sort((a, b) => a.distanceKm - b.distanceKm);
  } catch {
    // Fallback while backend endpoint is pending
    return generateMockBloodBanks(bloodType).sort(
      (a, b) => a.distanceKm - b.distanceKm,
    );
  }
}
