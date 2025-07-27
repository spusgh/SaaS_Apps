// loan.service.ts
import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../environments/environment';

export interface Loan {
  loanId: string;
  customerName: string;
  propertyAddress: string;
  originationDate: string;
  maturityDate: string;
  loanAmount: number;
  remainingBalance: number;
  interestRate: number;
  monthlyPayment: number;
  status: string;
  productName: string;
  productType: string;
  securityName: string;
  servicerName: string;
  currentStatus: string;
}

export interface SearchFilters {
  customerName?: string;
  status?: string;
  productType?: string;
  servicerName?: string;
  minLoanAmount?: number;
  maxLoanAmount?: number;
  originationDateFrom?: string;
  originationDateTo?: string;
}

export interface ApiResponse<T> {
  data: T;
  total: number;
  page: number;
  pageSize: number;
}

@Injectable({
  providedIn: 'root'
})
export class LoanService {
  private apiUrl = `${environment.apiUrl}/api/loans`;

  constructor(private http: HttpClient) {}

  searchLoans(filters: SearchFilters, page: number = 1, pageSize: number = 100): Observable<Loan[]> {
    let params = new HttpParams();
    
    // Add pagination
    params = params.set('page', page.toString());
    params = params.set('page_size', pageSize.toString());

    // Add filters
    if (filters.customerName) {
      params = params.set('customer_name', filters.customerName);
    }
    if (filters.status) {
      params = params.set('status', filters.status);
    }
    if (filters.productType) {
      params = params.set('product_type', filters.productType);
    }
    if (filters.servicerName) {
      params = params.set('servicer_name', filters.servicerName);
    }
    if (filters.minLoanAmount) {
      params = params.set('min_loan_amount', filters.minLoanAmount.toString());
    }
    if (filters.maxLoanAmount) {
      params = params.set('max_loan_amount', filters.maxLoanAmount.toString());
    }
    if (filters.originationDateFrom) {
      params = params.set('origination_date_from', filters.originationDateFrom);
    }
    if (filters.originationDateTo) {
      params = params.set('origination_date_to', filters.originationDateTo);
    }

    return this.http.get<Loan[]>(`${this.apiUrl}/search`, { params });
  }

  getLoanById(loanId: string): Observable<Loan> {
    return this.http.get<Loan>(`${this.apiUrl}/${loanId}`);
  }

  getStatistics(): Observable<any> {
    return this.http.get(`${this.apiUrl}/statistics`);
  }
}

