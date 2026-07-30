'use client';

import { ReactNode } from 'react';
import { useAdminGuard } from '@/lib/hooks/useAdminGuard';

export default function AdminLayout({ children }: { children: ReactNode }) {
  useAdminGuard();

  return <>{children}</>;
}
