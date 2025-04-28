# Bakery E-commerce Technical Documentation

## Project Overview
A React-based e-commerce platform built with Vite, TypeScript, and Tailwind CSS, designed for a bakery business. The application allows customers to browse products, place orders, and schedule pickups/deliveries.

## Tech Stack
- **Framework**: React 18.3.1 with TypeScript
- **Build Tool**: Vite
- **Styling**: Tailwind CSS
- **UI Components**: Shadcn/ui
- **Routing**: React Router DOM 6.26.2
- **State Management**: React Context + TanStack Query 5.56.2
- **Animations**: Framer Motion 12.9.2
- **Icons**: Lucide React 0.462.0
- **Toast Notifications**: Sonner 1.5.0

## Project Structure
```
src/
├── components/           # Reusable UI components
│   ├── ui/              # Shadcn UI components
│   ├── CartContext.tsx  # Shopping cart state management
│   ├── ProductCard.tsx  # Product display component
│   └── ...
├── pages/               # Route components
│   ├── Index.tsx        # Homepage
│   ├── Products.tsx     # Product catalog
│   ├── ProductDetail.tsx# Single product view
│   ├── Cart.tsx        # Shopping cart
│   └── Checkout.tsx     # Checkout process
├── data/               # Static data and types
│   └── products.ts     # Product catalog data
└── hooks/              # Custom React hooks
```

## Key Features

### 1. Product Catalog
- Grid-based product display with filtering
- Detailed product views with nutritional information
- Category-based filtering
- Responsive image handling

### 2. Shopping Cart
- Persistent cart state using localStorage
- Real-time cart updates
- Special instructions per item
- Quantity adjustments
- Total calculation with tax

### 3. Checkout Process
- Multi-step form validation
- Payment method selection
- Pickup/delivery scheduling
- Order confirmation
- Form validation with error messaging

### 4. UI/UX Features
- Responsive design for all screen sizes
- Smooth animations using Framer Motion
- Toast notifications for user feedback
- Accessible components from Shadcn/ui
- Loading states and error handling

## State Management
The application uses React Context for global state management:

### CartContext
```typescript
interface CartItem {
  id: number;
  name: string;
  price: number;
  image: string;
  quantity: number;
  specialInstructions?: string;
}

interface CartContextType {
  items: CartItem[];
  addToCart: (item: CartItem) => void;
  removeFromCart: (id: number) => void;
  updateQuantity: (id: number, quantity: number) => void;
  updateSpecialInstructions: (id: number, instructions: string) => void;
  clearCart: () => void;
  cartTotal: number;
  itemCount: number;
}
```

## Routing Structure
```typescript
<Routes>
  <Route path="/" element={<Index />} />
  <Route path="/products" element={<Products />} />
  <Route path="/product/:id" element={<ProductDetail />} />
  <Route path="/cart" element={<Cart />} />
  <Route path="/checkout" element={<Checkout />} />
  <Route path="/about" element={<About />} />
  <Route path="/contact" element={<Contact />} />
  <Route path="*" element={<NotFound />} />
</Routes>
```

## Data Flow
1. Products are loaded from static data in `data/products.ts`
2. Cart operations are handled through CartContext
3. User interactions trigger state updates
4. Changes persist to localStorage where appropriate
5. Form submissions trigger validation before processing

## Styling Guidelines
- Uses Tailwind CSS utility classes
- Custom color palette:
  - Primary: pastry-200 to pastry-800
  - Secondary: bakery-100 to bakery-900
  - Accent: crust
- Responsive breakpoints:
  - sm: 640px
  - md: 768px
  - lg: 1024px
  - xl: 1280px

## Performance Considerations
- Image optimization for product photos
- Cart state persistence in localStorage
- Lazy loading of route components
- Debounced form inputs
- Memoized component renders where appropriate

## Future Enhancements
1. Supabase Integration
   - User authentication
   - Order history
   - Product management
2. Payment Processing
   - Stripe integration
   - Multiple payment methods
3. Advanced Features
   - AI-powered recommendations
   - Loyalty program implementation
   - Email notifications
   - Order tracking

## Development Guidelines
1. Component Creation
   - Keep components small and focused
   - Use TypeScript for type safety
   - Follow React best practices
2. Styling
   - Use Tailwind CSS classes
   - Maintain consistent spacing
   - Follow responsive design patterns
3. State Management
   - Use Context for global state
   - Local state for component-specific data
   - Persist important data in localStorage


## Browser Support
- Modern browsers (Chrome, Firefox, Safari, Edge)
- Mobile browsers (iOS Safari, Android Chrome)
- Responsive design for all screen sizes

