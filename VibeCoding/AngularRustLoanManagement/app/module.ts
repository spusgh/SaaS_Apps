// app.module.ts
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { ReactiveFormsModule } from '@angular/forms';
import { HttpClientModule } from '@angular/common/http';
import { CommonModule } from '@angular/common';

import { AppComponent } from './app.component';
import { LoanSearchComponent } from './loan-search.component';
import { LoanService } from './loan.service';

@NgModule({
    declarations: [
        AppComponent,
        LoanSearchComponent
    ],
    imports: [
        BrowserModule,
        CommonModule,
        ReactiveFormsModule,
        HttpClientModule
    ],
    providers: [LoanService],
    bootstrap: [AppComponent]
})
export class AppModule { }