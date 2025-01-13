using FinTech_MauiApp.Models;
using FinTech_MauiApp.PageModels;

namespace FinTech_MauiApp.Pages
{
    public partial class MainPage : ContentPage
    {
        public MainPage(MainPageModel model)
        {
            InitializeComponent();
            BindingContext = model;
        }
    }
}