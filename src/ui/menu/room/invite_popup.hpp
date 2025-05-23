#pragma once
#include <defs/all.hpp>
#include <data/types/gd.hpp>

#include "player_list_cell.hpp"
#include <ui/general/list/list.hpp>

class InvitePopup : public geode::Popup<> {
public:
    constexpr static float POPUP_WIDTH = 420.f;
    constexpr static float POPUP_HEIGHT = 280.f;
    constexpr static float LIST_WIDTH = 340.f;
    constexpr static float LIST_HEIGHT = 220.f;

    static InvitePopup* create();

protected:
    using UserList = GlobedListLayer<PlayerListCell>;

    std::vector<PlayerPreviewAccountData> playerList;
    std::vector<PlayerPreviewAccountData> filteredPlayerList;

    LoadingCircle* loadingCircle = nullptr;
    UserList* listLayer = nullptr;
    cocos2d::CCMenu* buttonMenu;
    Ref<CCMenuItemSpriteExtra> clearSearchButton, settingsButton;

    cocos2d::CCMenu* roomBtnMenu = nullptr;
    bool isWaiting = false;

    bool setup() override;
    void onLoaded(bool stateChanged);
    void removeLoadingCircle();
    void reloadPlayerList(bool sendPacket = true);
    void addButtons();
    bool isLoading();
    void sortPlayerList();
    void applyFilter(std::string_view input);
};