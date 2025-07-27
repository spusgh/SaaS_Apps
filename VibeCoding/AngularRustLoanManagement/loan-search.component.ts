// loan-search.component.ts
import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { debounceTime, distinctUntilChanged } from 'rxjs/operators';
import { LoanService } from './loan.service';

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

@Component({
  selector: 'app-loan-search',
  template: `
    <div class="loan-search-container">
      <h2>Loan Search</h2>
      
      <form [formGroup]="searchForm" class="search-form">
        <div class="form-row">
          <div class="form-group">
            <label for="customerName">Customer Name</label>
            <input 
              type="text" 
              id="customerName"
              formControlName="customerName"
              class="form-control"
              placeholder="Enter customer name">
          </div>
          
          <div class="form-group">
            <label for="status">Status</label>
            <select id="status" formControlName="status" class="form-control">
              <option value="">All Statuses</option>
              <option value="Active">Active</option>
              <option value="Paid Off">Paid Off</option>
              <option value="Delinquent">Delinquent</option>
              <option value="Default">Default</option>
            </select>
          </div>
          
          <div class="form-group">
            <label for="productType">Product Type</label>
            <select id="productType" formControlName="productType" class="form-control">
              <option value="">All Product Types</option>
              <option value="Fixed Rate">Fixed Rate</option>
              <option value="Adjustable Rate">Adjustable Rate</option>
              <option value="Interest Only">Interest Only</option>
            </select>
          </div>
        </div>
        
        <div class="form-row">
          <div class="form-group">
            <label for="servicerName">Servicer</label>
            <input 
              type="text" 
              id="servicerName"
              formControlName="servicerName"
              class="form-control"
              placeholder="Enter servicer name">
          </div>
          
          <div class="form-group">
            <label for="minLoanAmount">Min Loan Amount</label>
            <input 
              type="number" 
              id="minLoanAmount"
              formControlName="minLoanAmount"
              class="form-control"
              placeholder="0">
          </div>
          
          <div class="form-group">
            <label for="maxLoanAmount">Max Loan Amount</label>
            <input 
              type="number" 
              id="maxLoanAmount"
              formControlName="maxLoanAmount"
              class="form-control"
              placeholder="1000000">
          </div>
        </div>
        
        <div class="form-row">
          <div class="form-group">
            <label for="originationDateFrom">Origination Date From</label>
            <input 
              type="date" 
              id="originationDateFrom"
              formControlName="originationDateFrom"
              class="form-control">
          </div>
          
          <div class="form-group">
            <label for="originationDateTo">Origination Date To</label>
            <input 
              type="date" 
              id="originationDateTo"
              formControlName="originationDateTo"
              class="form-control">
          </div>
          
          <div class="form-group">
            <button type="button" (click)="clearFilters()" class="btn btn-secondary">
              Clear Filters
            </button>
          </div>
        </div>
      </form>
      
      <div class="search-results">
        <div class="results-header">
          <h3>Search Results</h3>
          <span class="results-count">{{ loans.length }} loans found</span>
        </div>
        
        <div *ngIf="loading" class="loading">Loading...</div>
        
        <div *ngIf="error" class="error">{{ error }}</div>
        
        <div class="table-container">
          <table class="loans-table">
            <thead>
              <tr>
                <th>Loan ID</th>
                <th>Customer Name</th>
                <th>Property Address</th>
                <th>Loan Amount</th>
                <th>Remaining Balance</th>
                <th>Interest Rate</th>
                <th>Monthly Payment</th>
                <th>Status</th>
                <th>Product Type</th>
                <th>Servicer</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr *ngFor="let loan of loans" class="loan-row">
                <td>{{ loan.loanId }}</td>
                <td>{{ loan.customerName }}</td>
                <td>{{ loan.propertyAddress }}</td>
                <td>{{ loan.loanAmount | currency }}</td>
                <td>{{ loan.remainingBalance | currency }}</td>
                <td>{{ loan.interestRate }}%</td>
                <td>{{ loan.monthlyPayment | currency }}</td>
                <td>
                  <span class="status-badge" [ngClass]="getStatusClass(loan.status)">
                    {{ loan.status }}
                  </span>
                </td>
                <td>{{ loan.productType }}</td>
                <td>{{ loan.servicerName }}</td>
                <td>
                  <button (click)="viewLoanDetails(loan)" class="btn btn-primary btn-sm">
                    View Details
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        
        <div *ngIf="loans.length === 0 && !loading" class="no-results">
          No loans found matching your criteria.
        </div>
      </div>
    </div>
  `,
  styles: [`
    .loan-search-container {
      padding: 20px;
      max-width: 1400px;
      margin: 0 auto;
    }
    
    .search-form {
      background: #f8f9fa;
      padding: 20px;
      border-radius: 8px;
      margin-bottom: 30px;
    }
    
    .form-row {
      display: flex;
      gap: 20px;
      margin-bottom: 20px;
      flex-wrap: wrap;
    }
    
    .form-group {
      flex: 1;
      min-width: 200px;
    }
    
    .form-group label {
      display: block;
      margin-bottom: 5px;
      font-weight: 500;
      color: #333;
    }
    
    .form-control {
      width: 100%;
      padding: 8px 12px;
      border: 1px solid #ddd;
      border-radius: 4px;
      font-size: 14px;
    }
    
    .form-control:focus {
      outline: none;
      border-color: #007bff;
      box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
    }
    
    .btn {
      padding: 8px 16px;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      font-size: 14px;
      transition: background-color 0.2s;
    }
    
    .btn-primary {
      background-color: #007bff;
      color: white;
    }
    
    .btn-primary:hover {
      background-color: #0056b3;
    }
    
    .btn-secondary {
      background-color: #6c757d;
      color: white;
    }
    
    .btn-secondary:hover {
      background-color: #545b62;
    }
    
    .btn-sm {
      padding: 4px 8px;
      font-size: 12px;
    }
    
    .results-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 20px;
    }
    
    .results-count {
      color: #666;
      font-size: 14px;
    }
    
    .table-container {
      overflow-x: auto;
      border-radius: 8px;
      box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }
    
    .loans-table {
      width: 100%;
      border-collapse: collapse;
      background: white;
    }
    
    .loans-table th,
    .loans-table td {
      padding: 12px;
      text-align: left;
      border-bottom: 1px solid #eee;
    }
    
    .loans-table th {
      background-color: #f8f9fa;
      font-weight: 600;
      color: #333;
      position: sticky;
      top: 0;
    }
    
    .loan-row:hover {
      background-color: #f8f9fa;
    }
    
    .status-badge {
      padding: 4px 8px;
      border-radius: 4px;
      font-size: 12px;
      font-weight: 500;
    }
    
    .status-active {
      background-color: #d4edda;
      color: #155724;
    }
    
    .status-paid-off {
      background-color: #cce7ff;
      color: #004085;
    }
    
    .status-delinquent {
      background-color: #fff3cd;
      color: #856404;
    }
    
    .status-default {
      background-color: #f8d7da;
      color: #721c24;
    }
    
    .loading {
      text-align: center;
      padding: 40px;
      color: #666;
    }
    
    .error {
      background-color: #f8d7da;
      color: #721c24;
      padding: 12px;
      border-radius: 4px;
      margin-bottom: 20px;
    }
    
    .no-results {
      text-align: center;
      padding: 40px;
      color: #666;
      background-color: #f8f9fa;
      border-radius: 8px;
    }
  `]
})
export class LoanSearchComponent implements OnInit {
  searchForm: FormGroup;
  loans: Loan[] = [];
  loading = false;
  error: string | null = null;

  constructor(private loanService: LoanService) {
    this.searchForm = new FormGroup({
      customerName: new FormControl(''),
      status: new FormControl(''),
      productType: new FormControl(''),
      servicerName: new FormControl(''),
      minLoanAmount: new FormControl(''),
      maxLoanAmount: new FormControl(''),
      originationDateFrom: new FormControl(''),
      originationDateTo: new FormControl('')
    });
  }

  ngOnInit() {
    // Initial load
    this.searchLoans();

    // Set up reactive search
    this.searchForm.valueChanges
      .pipe(
        debounceTime(300),
        distinctUntilChanged()
      )
      .subscribe(() => {
        this.searchLoans();
      });
  }

  searchLoans() {
    this.loading = true;
    this.error = null;

    const filters: SearchFilters = {};
    const formValue = this.searchForm.value;

    // Build filters object
    if (formValue.customerName) filters.customerName = formValue.customerName;
    if (formValue.status) filters.status = formValue.status;
    if (formValue.productType) filters.productType = formValue.productType;
    if (formValue.servicerName) filters.servicerName = formValue.servicerName;
    if (formValue.minLoanAmount) filters.minLoanAmount = formValue.minLoanAmount;
    if (formValue.maxLoanAmount) filters.maxLoanAmount = formValue.maxLoanAmount;
    if (formValue.originationDateFrom) filters.originationDateFrom = formValue.originationDateFrom;
    if (formValue.originationDateTo) filters.originationDateTo = formValue.originationDateTo;

    this.loanService.searchLoans(filters).subscribe({
      next: (loans) => {
        this.loans = loans;
        this.loading = false;
      },
      error: (error) => {
        this.error = 'Failed to load loans. Please try again.';
        this.loading = false;
        console.error('Search error:', error);
      }
    });
  }

  clearFilters() {
    this.searchForm.reset();
  }

  getStatusClass(status: string): string {
    return `status-${status.toLowerCase().replace(/\s+/g, '-')}`;
  }

  viewLoanDetails(loan: Loan) {
    // Navigate to loan details or open modal
    console.log('Viewing loan details:', loan);
  }
}