"use client";

import React from "react";
import { cn } from "@/lib/utils/cn";

interface LoadingSpinnerProps {
  /** Set true to center the spinner in the full viewport */
  fullPage?: boolean;
  /** Optional additional class names */
  className?: string;
  /** ARIA label for screen readers */
  label?: string;
}

/**
 * Shared loading spinner.
 * Use this as the consistent loading state for all React Query-powered pages.
 *
 * @example
 * if (isLoading) return <LoadingSpinner fullPage />;
 */
export function LoadingSpinner({
  fullPage = false,
  className,
  label = "Loading…",
}: LoadingSpinnerProps) {
  const wrapper = fullPage
    ? "fixed inset-0 flex items-center justify-center bg-white/60 z-50"
    : "flex items-center justify-center py-12";

  return (
    <div className={cn(wrapper, className)} role="status" aria-label={label}>
      <svg
        className="h-10 w-10 animate-spin text-[#D32F2F]"
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        aria-hidden="true"
      >
        <circle
          className="opacity-25"
          cx="12"
          cy="12"
          r="10"
          stroke="currentColor"
          strokeWidth="4"
        />
        <path
          className="opacity-75"
          fill="currentColor"
          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
        />
      </svg>
      <span className="sr-only">{label}</span>
    </div>
  );
}
