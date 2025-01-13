using CommunityToolkit.Mvvm.Input;
using FinTech_MauiApp.Models;

namespace FinTech_MauiApp.PageModels
{
    public interface IProjectTaskPageModel
    {
        IAsyncRelayCommand<ProjectTask> NavigateToTaskCommand { get; }
        bool IsBusy { get; }
    }
}