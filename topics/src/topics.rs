pub const ADMIN_ACTION_GET_ALL_BY_FILTER_LIST_TOPIC: &'static str = "admin_action_getAllByFilterList";
pub const ADMIN_ACTION_GET_ACTION_LIST_TOPIC: &'static str = "admin_action_getActionList";
pub const ADMIN_ACTION_UNDO_TOPIC: &'static str = "admin_action_undo";
pub const ADMIN_ACTION_DELETE_BY_ACTION_ID_TOPIC: &'static str = "admin_action_deleteByActionId";
pub const ADMIN_ACTION_GET_METHOD_LIST_TOPIC: &'static str = "admin_action_getMethodList";
pub const ADMIN_MAIL_TEMPLATE_CREATE_TOPIC: &'static str = "admin_mailTemplate_create";
pub const ADMIN_MAIL_TEMPLATE_DELETE_TOPIC: &'static str = "admin_mailTemplate_delete";
pub const ADMIN_MAIL_TEMPLATE_GET_BY_KEY_TOPIC: &'static str = "admin_mailTemplate_getByKey";
pub const ADMIN_MAIL_TEMPLATE_CHANGE_TOPIC: &'static str = "admin_mailTemplate_change";
pub const ADMIN_MAIL_TEMPLATE_GET_ALL_TOPIC: &'static str = "admin_mailTemplate_getAll";
pub const ADMIN_OPTION_SET_TOPIC: &'static str = "admin_option_set";
pub const ADMIN_OPTION_UNSET_TOPIC: &'static str = "admin_option_unset";
pub const ADMIN_OPTION_GET_TOPIC: &'static str = "admin_option_get";
pub const ADMIN_OPTION_GET_LIST_TOPIC: &'static str = "admin_option_getList";
pub const ADMIN_OPTION_GET_BY_SCOPE_TOPIC: &'static str = "admin_option_getByScope";
pub const ADMIN_OPTION_GET_NICKNAME_PRICE_TOPIC: &'static str = "admin_option_getNicknamePrice";
pub const ADMIN_OPTION_GET_REFERRAL_REWARD_TOPIC: &'static str = "admin_option_getReferralReward";
pub const ADMIN_OPTION_GET_REFERRAL_FEE_TOPIC: &'static str = "admin_option_getReferralFee";
pub const ADMIN_PERMISSION_ADD_TOPIC: &'static str = "admin_permission_add";
pub const ADMIN_PERMISSION_DELETE_TOPIC: &'static str = "admin_permission_delete";
pub const ADMIN_PERMISSION_UPDATE_PERMISSIONS_TOPIC: &'static str = "admin_permission_updatePermissions";
pub const ADMIN_ROLE_ADD_TOPIC: &'static str = "admin_role_add";
pub const ADMIN_ROLE_DELETE_TOPIC: &'static str = "admin_role_delete";
pub const ADMIN_ROLE_GET_ALL_ROLES_TOPIC: &'static str = "admin_role_getAllRoles";
pub const ADMIN_ROLE_GET_PERMISSIONS_BY_ROLE_NAME_TOPIC: &'static str = "admin_role_getPermissionsByRoleName";
pub const ADMIN_ROLE_GET_PERMISSIONS_BY_ROLE_ID_TOPIC: &'static str = "admin_role_getPermissionsByRoleId";
pub const ADMIN_ROLE_UPDATE_TOPIC: &'static str = "admin_role_update";
pub const ADMIN_ROLE_GET_ROLE_TOPIC: &'static str = "admin_role_getRole";
pub const ADMIN_ROLE_GET_ROLE_ID_BY_ROLE_NAME_TOPIC: &'static str = "admin_role_getRoleIdByRoleName";
pub const ADMIN_ROLE_GET_ROLE_BY_NAME_TOPIC: &'static str = "admin_role_getRoleByName";
pub const ADMIN_ROLE_GET_USERS_BY_ROLE_TOPIC: &'static str = "admin_role_getUsersByRole";
pub const ADMIN_ROLE_GET_ROLE_STRICT_TOPIC: &'static str = "admin_role_getRoleStrict";
pub const ADMIN_USER_ROLE_ADD_TOPIC: &'static str = "admin_userRole_add";
pub const ADMIN_USER_ROLE_DELETE_TOPIC: &'static str = "admin_userRole_delete";
pub const ADMIN_USER_ROLE_GET_USER_ROLE_TOPIC: &'static str = "admin_userRole_getUserRole";
pub const ADMIN_USER_ROLE_GET_MY_ROLE_TOPIC: &'static str = "admin_userRole_getMyRole";
pub const ADMIN_USER_ROLE_DELETE_USER_ROLE_TOPIC: &'static str = "admin_userRole_deleteUserRole";
pub const ADMIN_USER_ROLE_UPDATE_USER_ROLE_TOPIC: &'static str = "admin_userRole_updateUserRole";
pub const ADMIN_USER_ROLE_GET_ALL_TOPIC: &'static str = "admin_userRole_getAll";
pub const ADMIN_USER_ROLE_GET_ALL_WITH_NICKNAME_TOPIC: &'static str = "admin_userRole_getAllWithNickname";
pub const AUTH_ADMIN_LOGIN_TOPIC: &'static str = "auth_admin_login";
pub const AUTH_ADMIN_ADD_ROLE_TO_USER_TOPIC: &'static str = "auth_admin_addRoleToUser";
pub const AUTH_ADMIN_DELETE_USER_ROLE_TOPIC: &'static str = "auth_admin_deleteUserRole";
pub const AUTH_ADMIN_UPDATE_USER_ROLE_TOPIC: &'static str = "auth_admin_updateUserRole";
pub const AUTH_ADMIN_GET_USER_ROLE_TOPIC: &'static str = "auth_admin_getUserRole";
pub const AUTH_AUTH_LOGIN_TOPIC: &'static str = "auth_auth_login";
pub const AUTH_AUTH_CHANGE_PASSWORD_TOPIC: &'static str = "auth_auth_changePassword";
pub const AUTH_AUTH_SEND_REQUEST_TO_RESET_PASSWORD_TOPIC: &'static str = "auth_auth_sendRequestToResetPassword";
pub const AUTH_AUTH_RESET_PASSWORD_TOPIC: &'static str = "auth_auth_resetPassword";
pub const AUTH_AUTH_GET_ACCESS_TOKEN_BY_REFRESH_TOPIC: &'static str = "auth_auth_getAccessTokenByRefresh";
pub const AUTH_AUTH_GET_ALL_MY_ACTIVE_SESSIONS_TOPIC: &'static str = "auth_auth_getAllMyActiveSessions";
pub const AUTH_AUTH_CONFIRM_PASSWORD_TOPIC: &'static str = "auth_auth_confirmPassword";
pub const AUTH_AUTH_DELETE_MY_USER_TOPIC: &'static str = "auth_auth_deleteMyUser";
pub const AUTH_AUTH_GET_USER_INFO_BY_TOKEN_TOPIC: &'static str = "auth_auth_getUserInfoByToken";
pub const AUTH_AUTH_CREATE_REQUEST_TO_CONFIRM_EMAIL_TOPIC: &'static str = "auth_auth_createRequestToConfirmEmail";
pub const AUTH_AUTH_CONFIRM_EMAIL_BY_CODE_TOPIC: &'static str = "auth_auth_confirmEmailByCode";
pub const AUTH_AUTH_CONFIRM_EMAIL_BY_HASH_TOPIC: &'static str = "auth_auth_confirmEmailByHash";
pub const AUTH_AUTH_DELETE_EMAIL_REQUEST_TOPIC: &'static str = "auth_auth_deleteEmailRequest";
pub const AUTH_AUTH_SET_PASSWORD_TOPIC: &'static str = "auth_auth_setPassword";
pub const AUTH_AUTH_DISABLE_USER_TOPIC: &'static str = "auth_auth_disableUser";
pub const AUTH_AUTH_RESENT_CONFIRMATION_MAIL_TOPIC: &'static str = "auth_auth_resentConfirmationMail";
pub const AUTH_AUTH_CREATE_REQUEST_TO_CHANGE_EMAIL_TOPIC: &'static str = "auth_auth_createRequestToChangeEmail";
pub const AUTH_AUTH_CHANGE_MY_EMAIL_TOPIC: &'static str = "auth_auth_changeMyEmail";
pub const AUTH_AUTH_LOGIN_WITH_FACEBOOK_TOPIC: &'static str = "auth_auth_loginWithFacebook";
pub const AUTH_AUTH_LOGOUT_TOPIC: &'static str = "auth_auth_logout";
pub const AUTH_AUTH_EMPTY_METHOD_TOPIC: &'static str = "auth_auth_emptyMethod";
pub const AUTH_AUTH_ASSIGN_PROVIDER_ACCOUNT_TO_DEVICE_ID_TOPIC: &'static str =
    "auth_auth_assignProviderAccountToDeviceId";
pub const AUTH_BAN_BAN_TOPIC: &'static str = "auth_ban_ban";
pub const AUTH_BAN_UN_BAN_TOPIC: &'static str = "auth_ban_unBan";
pub const AUTH_BAN_GET_USER_BAN_TOPIC: &'static str = "auth_ban_getUserBan";
pub const AUTH_CLIENTS_GET_OR_CREATE_INTERNAL_USER_ID_TOPIC: &'static str = "auth_clients_getOrCreateInternalUserId";
pub const AUTH_CLIENTS_UPDATE_INTERNAL_ID_BY_DEVICE_ID_TOPIC: &'static str = "auth_clients_updateInternalIdByDeviceId";
pub const AUTH_CLIENTS_GET_CLIENT_INFO_TOPIC: &'static str = "auth_clients_getClientInfo";
pub const AUTH_CLIENTS_ASSIGN_INTERNAL_ID_TO_PROVIDER_ID_TOPIC: &'static str =
    "auth_clients_assignInternalIdToProviderId";
pub const AUTH_CLIENTS_UPDATE_PROVIDER_STATUS_BY_PROVIDER_ID_TOPIC: &'static str =
    "auth_clients_updateProviderStatusByProviderId";
pub const AUTH_CLIENTS_UPDATE_PROVIDER_STATUS_BY_INTERNAL_USER_ID_TOPIC: &'static str =
    "auth_clients_updateProviderStatusByInternalUserId";
pub const AUTH_CLIENTS_GET_PROVIDER_BY_INTERNAL_USER_ID_TOPIC: &'static str =
    "auth_clients_getProviderByInternalUserId";
pub const AUTH_CLIENTS_GET_CLIENT_STATUS_BY_INTERNAL_USER_ID_TOPIC: &'static str =
    "auth_clients_getClientStatusByInternalUserId";
pub const AUTH_CLIENTS_LOGOUT_DEVICE_BY_KEYCLOAK_ID_TOPIC: &'static str = "auth_clients_logoutDeviceByKeycloakId";
pub const AUTH_CLIENTS_GET_DEVICE_ID_BY_INTERNAL_ID_TOPIC: &'static str = "auth_clients_getDeviceIdByInternalId";
pub const AUTH_CLIENTS_IS_NEW_CLIENT_TOPIC: &'static str = "auth_clients_isNewClient";
pub const AUTH_CLIENTS_IS_AUTHORIZED_TOPIC: &'static str = "auth_clients_isAuthorized";
pub const AUTH_CLIENTS_GET_LOGIN_DEVICE_BY_USER_ID_TOPIC: &'static str = "auth_clients_getLoginDeviceByUserId";
pub const AUTH_CLIENTS_LOGOUT_ALL_DEVICES_TOPIC: &'static str = "auth_clients_logoutAllDevices";
pub const AUTH_CLIENTS_DELETE_ACCOUNT_TOPIC: &'static str = "auth_clients_deleteAccount";
pub const AUTH_SESSIONS_ADD_NEW_DEVICE_SESSION_TOPIC: &'static str = "auth_sessions_addNewDeviceSession";
pub const AUTH_SESSIONS_ADD_NEW_WEB_SESSION_TOPIC: &'static str = "auth_sessions_addNewWebSession";
pub const AUTH_SESSIONS_DELETE_ALL_BY_INTERNAL_USER_ID_TOPIC: &'static str = "auth_sessions_deleteAllByInternalUserId";
pub const AUTH_SESSIONS_DELETE_ALL_MY_TOPIC: &'static str = "auth_sessions_deleteAllMy";
pub const AUTH_SESSIONS_DELETE_BY_SESSION_ID_TOPIC: &'static str = "auth_sessions_deleteBySessionId";
pub const AUTH_TWO_FACTOR_GENERATE_SECRET_TOPIC: &'static str = "auth_twoFactor_generateSecret";
pub const AUTH_TWO_FACTOR_VERIFY_SECRET_TOPIC: &'static str = "auth_twoFactor_verifySecret";
pub const AUTH_TWO_FACTOR_VERIFY_TOKEN_TOPIC: &'static str = "auth_twoFactor_verifyToken";
pub const AUTH_TWO_FACTOR_DELETE_SECRET_TOPIC: &'static str = "auth_twoFactor_deleteSecret";
pub const AUTH_TWO_FACTOR_CAN_GENERATE_SECRET_TOPIC: &'static str = "auth_twoFactor_canGenerateSecret";
pub const BINANCE_WALLET_WALLET_CREATE_WALLET_TOPIC: &'static str = "binance-wallet_wallet_createWallet";
pub const CATS_LOBBY_ACHIEVEMENT_COMPLETE_TOPIC: &'static str = "cats_lobby_achievementComplete";
pub const CATS_LOBBY_GET_ALL_ACHIEVEMENTS_TOPIC: &'static str = "cats_lobby_getAllAchievements";
pub const CATS_LOBBY_GET_USERS_ACHIEVEMENTS_TOPIC: &'static str = "cats_lobby_getUsersAchievements";
pub const CATS_LOBBY_ADVENTURE_MAP_LEVEL_UP_TOPIC: &'static str = "cats_lobby_adventureMapLevelUp";
pub const CATS_LOBBY_GET_ADVENTURE_MAP_STATS_TOPIC: &'static str = "cats_lobby_getAdventureMapStats";
pub const CATS_LOBBY_GET_APPLICATION_SETTINGS_TOPIC: &'static str = "cats_lobby_getApplicationSettings";
pub const CATS_LOBBY_CRAFT_BINGO_RECIPE_TOPIC: &'static str = "cats_lobby_craftBingoRecipe";
pub const CATS_LOBBY_GET_BINGO_DATA_TOPIC: &'static str = "cats_lobby_getBingoData";
pub const CATS_LOBBY_GET_USERS_CARDS_TOPIC: &'static str = "cats_lobby_getUsersCards";
pub const CATS_LOBBY_UPGRADE_CARD_TOPIC: &'static str = "cats_lobby_upgradeCard";
pub const CATS_LOBBY_GET_CLIENT_VERSION_TOPIC: &'static str = "cats_lobby_getClientVersion";
pub const CATS_LOBBY_GET_CONFIG_BY_ID_TOPIC: &'static str = "cats_lobby_getConfigById";
pub const CATS_LOBBY_UPDATE_CONFIG_TOPIC: &'static str = "cats_lobby_updateConfig";
pub const CATS_LOBBY_BUY_CAT_TOPIC: &'static str = "cats_lobby_buyCat";
pub const CATS_LOBBY_BUY_SCIENCE_TOPIC: &'static str = "cats_lobby_buyScience";
pub const CATS_LOBBY_BUY_SKILL_TOPIC: &'static str = "cats_lobby_buySkill";
pub const CATS_LOBBY_BUY_USUAL_LOOTBOX_TOPIC: &'static str = "cats_lobby_buyUsualLootbox";
pub const CATS_LOBBY_GET_BINGO_JACKPOT_POOL_TOPIC: &'static str = "cats_lobby_getBingoJackpotPool";
pub const CATS_LOBBY_GET_BINGO_JACKPOT_WINNERS_INFO_TOPIC: &'static str = "cats_lobby_getBingoJackpotWinnersInfo";
pub const CATS_LOBBY_GET_LOOTBOX_JACKPOT_WINNERS_INFO_TOPIC: &'static str = "cats_lobby_getLootboxJackpotWinnersInfo";
pub const CATS_LOBBY_GET_LOOTBOX_JACKPOT_POOL_TOPIC: &'static str = "cats_lobby_getLootboxJackpotPool";
pub const CATS_LOBBY_GET_PREVIOUS_TOURNAMENT_STATS_TOPIC: &'static str = "cats_lobby_getPreviousTournamentStats";
pub const CATS_LOBBY_GET_SOFT_LEADERBOARD_STATS_TOPIC: &'static str = "cats_lobby_getSoftLeaderboardStats";
pub const CATS_LOBBY_GET_TOURNAMENT_LEADERBOARD_STATS_TOPIC: &'static str = "cats_lobby_getTournamentLeaderboardStats";
pub const CATS_LOBBY_GET_LOOTBOXES_INFO_TOPIC: &'static str = "cats_lobby_getLootboxesInfo";
pub const CATS_LOBBY_GET_USUAL_LOOTBOX_TIME_TOPIC: &'static str = "cats_lobby_getUsualLootboxTime";
pub const CATS_LOBBY_OPEN_LOOTBOX_TOPIC: &'static str = "cats_lobby_openLootbox";
pub const CATS_LOBBY_GET_AVAILABLE_MEMBERSHIP_AMOUNT_TOPIC: &'static str = "cats_lobby_getAvailableMembershipAmount";
pub const CATS_LOBBY_GET_OFFERS_AND_BONUSES_TOPIC: &'static str = "cats_lobby_getOffersAndBonuses";
pub const CATS_LOBBY_REFUSE_OFFER_TOPIC: &'static str = "cats_lobby_refuseOffer";
pub const CATS_LOBBY_SAVE_TUTORIAL_PROGRESS_TOPIC: &'static str = "cats_lobby_saveTutorialProgress";
pub const CATS_LOBBY_SAVE_USERS_PROGRESS_TOPIC: &'static str = "cats_lobby_saveUsersProgress";
pub const CATS_LOBBY_GET_USERS_REFERRALS_TOPIC: &'static str = "cats_lobby_getUsersReferrals";
pub const CATS_LOBBY_SET_REFERRER_TOPIC: &'static str = "cats_lobby_setReferrer";
pub const CATS_LOBBY_GET_SERVER_TIME_TOPIC: &'static str = "cats_lobby_getServerTime";
pub const CATS_LOBBY_KEEP_ALIVE_TOPIC: &'static str = "cats_lobby_keepAlive";
pub const CATS_LOBBY_GET_SPIN_REWARDS_TOPIC: &'static str = "cats_lobby_getSpinRewards";
pub const CATS_LOBBY_USE_SPIN_TOPIC: &'static str = "cats_lobby_useSpin";
pub const CATS_LOBBY_COMPLETE_TASK_TOPIC: &'static str = "cats_lobby_completeTask";
pub const CATS_LOBBY_GET_CURRENT_TASKS_TOPIC: &'static str = "cats_lobby_getCurrentTasks";
pub const CATS_LOBBY_GET_USERS_COMPLETED_TASKS_TOPIC: &'static str = "cats_lobby_getUsersCompletedTasks";
pub const CATS_LOBBY_USER_VALIDATION_TOPIC: &'static str = "cats_lobby_userValidation";
pub const CATS_LOBBY_GET_BAD_TRANSACTIONS_TOPIC: &'static str = "cats_lobby_getBadTransactions";
pub const CATS_LOBBY_GET_USER_ID_BY_TRANSACTION_ID_TOPIC: &'static str = "cats_lobby_getUserIdByTransactionId";
pub const CATS_LOBBY_GET_USERS_INAPP_PURCHASES_TOPIC: &'static str = "cats_lobby_getUsersInappPurchases";
pub const CATS_LOBBY_RESET_DEVICE_ID_TOPIC: &'static str = "cats_lobby_resetDeviceId";
pub const CATS_LOBBY_UNLINK_ACCOUNT_TOPIC: &'static str = "cats_lobby_unlinkAccount";
pub const CATS_LOBBY_GET_PROFILE_TOPIC: &'static str = "cats_lobby_getProfile";
pub const CATS_LOBBY_GET_SOUL_PRICE_TOPIC: &'static str = "cats_lobby_getSoulPrice";
pub const CATS_LOBBY_PURCHASE_VALIDATION_TOPIC: &'static str = "cats_lobby_purchaseValidation";
pub const CATS_LOBBY_RETRIEVE_USERS_PROGRESS_TOPIC: &'static str = "cats_lobby_retrieveUsersProgress";
pub const CATS_LOBBY_SHARE_SCORE_TOPIC: &'static str = "cats_lobby_shareScore";
pub const CATS_LOBBY_SET_USER_APP_METRICA_DEVICE_ID_TOPIC: &'static str = "cats_lobby_setUserAppMetricaDeviceId";
pub const CATS_LOBBY_GET_JACKPOT_WINNERS_INFO_TOPIC: &'static str = "cats_lobby_getJackpotWinnersInfo";
pub const CATS_LOBBY_GET_PROFILE_INFO_TOPIC: &'static str = "cats_lobby_getProfileInfo";
pub const CATS_LOBBY_APPLICATION_INITIALIZATION_TOPIC: &'static str = "cats_lobby_applicationInitialization";
pub const CATS_AND_DRAGONS_WRAPPER_GET_NICKNAME_CHANGE_PRICE_TOPIC: &'static str =
    "catsAndDragons_wrapper_getNicknameChangePrice";
pub const CATS_AND_DRAGONS_WRAPPER_NICKNAME_CHANGE_TOPIC: &'static str = "catsAndDragons_wrapper_nicknameChange";
pub const CATS_AND_DRAGONS_WRAPPER_COLLECT_NICKNAMES_TOPIC: &'static str = "catsAndDragons_wrapper_collectNicknames";
pub const CATS_AND_DRAGONS_WRAPPER_GET_USER_ID_BY_DEVICE_ID_TOPIC: &'static str =
    "catsAndDragons_wrapper_getUserIdByDeviceId";
pub const CATS_AND_DRAGONS_WRAPPER_GET_REFERRALS_TOPIC: &'static str = "catsAndDragons_wrapper_getReferrals";
pub const CATS_AND_DRAGONS_WRAPPER_GET_NICKNAME_TOPIC: &'static str = "catsAndDragons_wrapper_getNickname";
pub const CATS_AND_DRAGONS_WRAPPER_GET_USER_INFO_TOPIC: &'static str = "catsAndDragons_wrapper_getUserInfo";
pub const CATS_AND_DRAGONS_WRAPPER_GET_MEMBERSHIPS_AND_PRICES_AMOUNT_TOPIC: &'static str =
    "catsAndDragons_wrapper_getMembershipsAndPricesAmount";
pub const CATS_AND_DRAGONS_WRAPPER_SET_MEMBERSHIP_TOPIC: &'static str = "catsAndDragons_wrapper_setMembership";
pub const CATS_AND_DRAGONS_WRAPPER_GET_USER_MEMBERSHIP_INFO_TOPIC: &'static str =
    "catsAndDragons_wrapper_getUserMembershipInfo";
pub const CATS_AND_DRAGONS_WRAPPER_EMPTY_METHOD_TOPIC: &'static str = "catsAndDragons_wrapper_emptyMethod";
pub const CRON_CRON_CREATE_TOPIC: &'static str = "cron_cron_create";
pub const CRON_CRON_UPDATE_TOPIC: &'static str = "cron_cron_update";
pub const CRON_CRON_GET_TOPIC: &'static str = "cron_cron_get";
pub const CRON_CRON_DELETE_TOPIC: &'static str = "cron_cron_delete";
pub const EMAIL_EMAIL_FIND_ALL_TOPIC: &'static str = "email_email_findAll";
pub const EMAIL_EMAIL_CREATE_CRON_JOB_TOPIC: &'static str = "email_email_createCronJob";
pub const EMAIL_EMAIL_CREATE_AND_SEND_TOPIC: &'static str = "email_email_createAndSend";
pub const EMAIL_EMAIL_SEND_NOT_SENT_LETTERS_TOPIC: &'static str = "email_email_sendNotSentLetters";
pub const EMAIL_EMAIL_SEND_IN_PROCESS_LETTERS_TOPIC: &'static str = "email_email_sendInProcessLetters";
pub const GAME_BALANCER_GAME_BALANCER_ADD_PLAYER_IN_SEARCH_TOPIC: &'static str =
    "gameBalancer_gameBalancer_addPlayerInSearch";
pub const GAME_BALANCER_GAME_BALANCER_DISCONNECT_PLAYER_TOPIC: &'static str =
    "gameBalancer_gameBalancer_disconnectPlayer";
pub const GAME_BALANCER_GAME_BALANCER_DELETE_PLAYERS_TOPIC: &'static str = "gameBalancer_gameBalancer_deletePlayers";
pub const GAME_BALANCER_GAME_BALANCER_STOP_SEARCHING_TOPIC: &'static str = "gameBalancer_gameBalancer_stopSearching";
pub const GAME_BALANCER_GAME_BALANCER_GAME_STARTED_NOTIFICATION_TOPIC: &'static str =
    "gameBalancer_gameBalancer_gameStartedNotification";
pub const GOOGLE_PLAY_PURCHASE_SUBSCRIBE_TOPIC: &'static str = "google-play_purchase_subscribe";
pub const GOOGLE_PLAY_PURCHASE_PURCHASE_TOPIC: &'static str = "google-play_purchase_purchase";
pub const GOOGLE_PLAY_PURCHASE_CHECK_UNCONFIRMED_PURCHASE_TOPIC: &'static str =
    "google-play_purchase_checkUnconfirmedPurchase";
pub const GOOGLE_PLAY_PURCHASE_SET_MEMBERSHIP_TOPIC: &'static str = "google-play_purchase_setMembership";
pub const LOBBY_HEROES_GET_HEROES_LIST_BY_USER_ID_TOPIC: &'static str = "lobby_heroes_getHeroesListByUserId";
pub const LOBBY_HEROES_GET_ACTIVE_HERO_BY_USER_ID_TOPIC: &'static str = "lobby_heroes_getActiveHeroByUserId";
pub const LOBBY_HEROES_HERO_ADDED_NOTIFICATION_TOPIC: &'static str = "lobby_heroes_heroAddedNotification";
pub const LOBBY_INVENTORY_ADDED_ITEM_NOTIFICATION_TOPIC: &'static str = "lobby_inventory_addedItemNotification";
pub const LOBBY_INVENTORY_GET_ITEMS_LIST_TOPIC: &'static str = "lobby_inventory_getItemsList";
pub const LOBBY_LOBBY_START_GAME_TOPIC: &'static str = "lobby_lobby_startGame";
pub const LOBBY_LOOT_BOXES_OPEN_TUTORIAL_LOOT_BOX_TOPIC: &'static str = "lobby_lootBoxes_openTutorialLootBox";
pub const LOBBY_REGIONS_GET_TOPIC: &'static str = "lobby_regions_get";
pub const LOBBY_REGIONS_SET_TOPIC: &'static str = "lobby_regions_set";
pub const LOBBY_SETTINGS_GET_TOPIC: &'static str = "lobby_settings_get";
pub const LOBBY_SETTINGS_SET_TOPIC: &'static str = "lobby_settings_set";
pub const LOBBY_STATS_EQUIP_ITEM_TOPIC: &'static str = "lobby_stats_equipItem";
pub const LOBBY_STATS_EQUIP_HERO_TOPIC: &'static str = "lobby_stats_equipHero";
pub const LOBBY_STATS_UNEQUIP_ITEM_TOPIC: &'static str = "lobby_stats_unequipItem";
pub const LOBBY_USER_GET_USER_DATA_TOPIC: &'static str = "lobby_user_getUserData";
pub const LOBBY_USER_UPDATE_USERNAME_TOPIC: &'static str = "lobby_user_updateUsername";
pub const LOBBY_USER_GET_LEADER_BOARD_TOPIC: &'static str = "lobby_user_getLeaderBoard";
pub const LOBBY_USER_UPDATE_PROFILE_IMAGE_TOPIC: &'static str = "lobby_user_updateProfileImage";
pub const MARKET_ITEMS_GET_LIST_OF_AVAILABLE_TOPIC: &'static str = "market_items_getListOfAvailable";
pub const MARKET_ITEMS_GET_PRICE_TOPIC: &'static str = "market_items_getPrice";
pub const NOTIFICATIONS_NOTIFICATIONS_CREATE_NEW_TOPIC: &'static str = "notifications_notifications_createNew";
pub const PRODUCT_FACTORY_ATTRIBUTE_ADD_ATTRIBUTES_TOPIC: &'static str = "productFactory_attribute_addAttributes";
pub const PRODUCT_FACTORY_ATTRIBUTE_UPDATE_ATTRIBUTES_TOPIC: &'static str = "productFactory_attribute_updateAttributes";
pub const PRODUCT_FACTORY_ATTRIBUTE_GET_ATTRIBUTE_LIST_TOPIC: &'static str =
    "productFactory_attribute_getAttributeList";
pub const PRODUCT_FACTORY_CARD_ADD_CARD_TOPIC: &'static str = "productFactory_card_addCard";
pub const PRODUCT_FACTORY_CARD_UPDATE_CARD_TOPIC: &'static str = "productFactory_card_updateCard";
pub const PRODUCT_FACTORY_CARD_GET_CARD_LIST_TOPIC: &'static str = "productFactory_card_getCardList";
pub const PRODUCT_FACTORY_COMMON_ATTRIBUTE_ADD_COMMON_ATTRIBUTE_TOPIC: &'static str =
    "productFactory_commonAttribute_addCommonAttribute";
pub const PRODUCT_FACTORY_COMMON_ATTRIBUTE_UPDATE_COMMON_ATTRIBUTE_TOPIC: &'static str =
    "productFactory_commonAttribute_updateCommonAttribute";
pub const PRODUCT_FACTORY_COMMON_ATTRIBUTE_GET_COMMON_ATTR_LIST_TOPIC: &'static str =
    "productFactory_commonAttribute_getCommonAttrList";
pub const PRODUCT_FACTORY_CUSTOM_ATTRIBUTE_ADD_CUSTOM_ATTRIBUTE_TOPIC: &'static str =
    "productFactory_customAttribute_addCustomAttribute";
pub const PRODUCT_FACTORY_CUSTOM_ATTRIBUTE_UPDATE_CUSTOM_ATTRIBUTE_TOPIC: &'static str =
    "productFactory_customAttribute_updateCustomAttribute";
pub const PRODUCT_FACTORY_CUSTOM_ATTRIBUTE_GET_CUSTOM_ATTR_LIST_TOPIC: &'static str =
    "productFactory_customAttribute_getCustomAttrList";
pub const PRODUCT_FACTORY_LOOTBOX_TYPE_ADD_LOOTBOX_TOPIC: &'static str = "productFactory_lootboxType_addLootbox";
pub const PRODUCT_FACTORY_LOOTBOX_TYPE_UPDATE_LOOTBOX_BY_LOOTBOX_ID_TOPIC: &'static str =
    "productFactory_lootboxType_updateLootboxByLootboxId";
pub const PRODUCT_FACTORY_LOOTBOX_TYPE_ADD_CARDS_TO_LOOTBOX_TOPIC: &'static str =
    "productFactory_lootboxType_addCardsToLootbox";
pub const PRODUCT_FACTORY_LOOTBOX_TYPE_ADD_SLOTS_TO_LOOTBOX_TOPIC: &'static str =
    "productFactory_lootboxType_addSlotsToLootbox";
pub const PRODUCT_FACTORY_LOOTBOX_TYPE_GET_LOOTBOX_LIST_TOPIC: &'static str =
    "productFactory_lootboxType_getLootboxList";
pub const PRODUCT_FACTORY_PRODUCT_MARK_HERO_AS_NFT_TOPIC: &'static str = "productFactory_product_markHeroAsNFT";
pub const PRODUCT_FACTORY_PRODUCT_GET_ALL_TOPIC: &'static str = "productFactory_product_getAll";
pub const PRODUCT_FACTORY_PRODUCT_TEST_CREATE_WALLET_TOPIC: &'static str = "productFactory_product_testCreateWallet";
pub const PRODUCT_FACTORY_PRODUCT_TEST_TO_REALIS_ADAPTER_TOPIC: &'static str =
    "productFactory_product_testToRealisAdapter";
pub const PRODUCT_FACTORY_PRODUCT_GENERATE_CUSTOM_PRODUCT_TOPIC: &'static str =
    "productFactory_product_generateCustomProduct";
pub const PRODUCT_FACTORY_PRODUCT_GET_PRODUCT_LIST_BY_PRODUCT_ID_LIST_TOPIC: &'static str =
    "productFactory_product_getProductListByProductIdList";
pub const PRODUCT_FACTORY_PRODUCT_ADD_HERO_TOPIC: &'static str = "productFactory_product_addHero";
pub const PRODUCT_FACTORY_PRODUCT_ADD_ITEM_TOPIC: &'static str = "productFactory_product_addItem";
pub const PRODUCT_FACTORY_PRODUCT_TYPE_ADD_TOPIC: &'static str = "productFactory_productType_add";
pub const PRODUCT_FACTORY_PRODUCT_TYPE_IS_EXIST_TOPIC: &'static str = "productFactory_productType_isExist";
pub const PRODUCT_FACTORY_PRODUCT_TYPE_UPDATE_TOPIC: &'static str = "productFactory_productType_update";
pub const PRODUCT_FACTORY_PRODUCT_TYPE_DELETE_TOPIC: &'static str = "productFactory_productType_delete";
pub const PRODUCT_FACTORY_PRODUCT_TYPE_GET_ALL_TOPIC: &'static str = "productFactory_productType_getAll";
pub const PRODUCT_FACTORY_PRODUCT_TYPE_GET_TOPIC: &'static str = "productFactory_productType_get";
pub const PRODUCT_FACTORY_PRODUCT_TYPE_GET_ALL_BY_TYPE_TOPIC: &'static str = "productFactory_productType_getAllByType";
pub const PRODUCT_FACTORY_PRODUCT_TYPE_GET_ALL_BY_RARITY_TOPIC: &'static str =
    "productFactory_productType_getAllByRarity";
pub const PRODUCT_FACTORY_PRODUCT_TYPE_GET_HASH_TOPIC: &'static str = "productFactory_productType_getHash";
pub const PRODUCT_FACTORY_PRODUCT_TYPE_GET_ATTRIBUTE_LIST_BY_PERSONAL_TYPE_TOPIC: &'static str =
    "productFactory_productType_getAttributeListByPersonalType";
pub const PRODUCT_FACTORY_RARITY_ADD_RARITY_CHANCES_TOPIC: &'static str = "productFactory_rarity_addRarityChances";
pub const PRODUCT_FACTORY_RARITY_UPDATE_RARITY_CHANCES_TOPIC: &'static str =
    "productFactory_rarity_updateRarityChances";
pub const PRODUCT_FACTORY_RARITY_GET_RARITY_LIST_TOPIC: &'static str = "productFactory_rarity_getRarityList";
pub const PRODUCT_FACTORY_SLOT_TYPE_ADD_SLOT_TYPE_TOPIC: &'static str = "productFactory_slotType_addSlotType";
pub const PRODUCT_FACTORY_SLOT_TYPE_GET_SLOT_TYPE_LIST_TOPIC: &'static str = "productFactory_slotType_getSlotTypeList";
pub const PROMO_CODES_USE_CODE_TOPIC: &'static str = "promo_codes_useCode";
pub const PROMO_CODES_CREATE_CODE_TOPIC: &'static str = "promo_codes_createCode";
pub const PROMO_CODES_GET_DATA_BY_CODE_STRICT_TOPIC: &'static str = "promo_codes_getDataByCodeStrict";
pub const PROMO_CODES_GET_LIST_STRICT_TOPIC: &'static str = "promo_codes_getListStrict";
pub const PROMO_CODES_DELETE_CODE_TOPIC: &'static str = "promo_codes_deleteCode";
pub const PURCHASE_BALANCE_INCREASE_USER_BALANCE_TOPIC: &'static str = "purchase_balance_increaseUserBalance";
pub const PURCHASE_BALANCE_DECREASE_USER_BALANCE_TOPIC: &'static str = "purchase_balance_decreaseUserBalance";
pub const PURCHASE_BALANCE_USER_BALANCE_CHANGED_NOTIFICATION_TOPIC: &'static str =
    "purchase_balance_userBalanceChangedNotification";
pub const PURCHASE_PRODUCT_PURCHASE_TOPIC: &'static str = "purchase_product_purchase";
pub const PURCHASE_PRODUCT_GET_HERO_LIST_TOPIC: &'static str = "purchase_product_getHeroList";
pub const PURCHASE_PRODUCT_GET_EQUIPMENT_LIST_TOPIC: &'static str = "purchase_product_getEquipmentList";
pub const PURCHASE_WALLET_GET_BY_USER_ID_TOPIC: &'static str = "purchase_wallet_getByUserId";
pub const REALIS_WALLET_MANAGER_GET_MY_ADDRESS_TOPIC: &'static str = "realis_walletManager_getMyAddress";
pub const REFERRAL_CODE_GET_REFERRAL_CODE_TOPIC: &'static str = "referral_code_getReferralCode";
pub const REFERRAL_CODE_GET_REFERRAL_LINK_TOPIC: &'static str = "referral_code_getReferralLink";
pub const REFERRAL_CODE_IS_CODE_EXIST_TOPIC: &'static str = "referral_code_isCodeExist";
pub const REFERRAL_REFERRAL_FIND_ALL_TOPIC: &'static str = "referral_referral_findAll";
pub const REFERRAL_REFERRAL_FIND_REFERRALS_BY_REFERRER_ID_TOPIC: &'static str =
    "referral_referral_findReferralsByReferrerId";
pub const REFERRAL_REFERRAL_GET_USER_DATA_TOPIC: &'static str = "referral_referral_getUserData";
pub const REFERRAL_REFERRAL_FIND_REFERRER_BY_REFERRAL_ID_STRICT_TOPIC: &'static str =
    "referral_referral_findReferrerByReferralIdStrict";
pub const REFERRAL_REFERRAL_GET_ALL_MY_REFERRAL_TOPIC: &'static str = "referral_referral_getAllMyReferral";
pub const REFERRAL_REFERRAL_GET_REFERRALS_TOPIC: &'static str = "referral_referral_getReferrals";
pub const REFERRAL_REFERRAL_SET_POTENTIAL_REFERRAL_TOPIC: &'static str = "referral_referral_setPotentialReferral";
pub const REFERRAL_REFERRAL_SET_REFERRAL_TOPIC: &'static str = "referral_referral_setReferral";
pub const REFERRAL_REWARD_ADD_REFERRAL_EXPENSE_TOPIC: &'static str = "referral_reward_addReferralExpense";
pub const REFERRAL_REWARD_MAKE_REWARD_REQUEST_TOPIC: &'static str = "referral_reward_makeRewardRequest";
pub const REFERRAL_REWARD_GET_AVAILABLE_REWARD_TOPIC: &'static str = "referral_reward_getAvailableReward";
pub const REFUND_BALANCES_GET_ALL_MY_TOPIC: &'static str = "refund_balances_getAllMy";
pub const REFUND_BALANCES_GET_ALL_TOPIC: &'static str = "refund_balances_getAll";
pub const REFUND_BALANCES_DELETE_TOPIC: &'static str = "refund_balances_delete";
pub const REFUND_BALANCES_GET_ALL_UNAVAILABLE_TOPIC: &'static str = "refund_balances_getAllUnavailable";
pub const REFUND_BALANCES_ADD_TOPIC: &'static str = "refund_balances_add";
pub const REFUND_BALANCES_GET_ALL_LOCKED_FUNDS_TOPIC: &'static str = "refund_balances_getAllLockedFunds";
pub const REFUND_ITEMS_GET_ALL_MY_TOPIC: &'static str = "refund_items_getAllMy";
pub const REFUND_ITEMS_GET_ALL_TOPIC: &'static str = "refund_items_getAll";
pub const REFUND_ITEMS_DELETE_TOPIC: &'static str = "refund_items_delete";
pub const REFUND_ITEMS_IS_AVAILABLE_TOPIC: &'static str = "refund_items_isAvailable";
pub const REFUND_ITEMS_GET_ALL_UNAVAILABLE_TOPIC: &'static str = "refund_items_getAllUnavailable";
pub const REFUND_ITEMS_ADD_TOPIC: &'static str = "refund_items_add";
pub const SOUL_ADAPTER_WALLET_GET_USER_ID_BY_ADDRESS_TOPIC: &'static str = "soul-adapter_wallet_getUserIdByAddress";
pub const SOUL_ADAPTER_WALLET_GET_MY_WALLET_TOPIC: &'static str = "soul-adapter_wallet_getMyWallet";
pub const SOUL_ADAPTER_WALLET_PROCESS_TRANSACTION_BY_HASH_TOPIC: &'static str =
    "soul-adapter_wallet_processTransactionByHash";
pub const SOUL_ADAPTER_WALLET_GET_BALANCE_BY_ADDRESS_TOPIC: &'static str = "soul-adapter_wallet_getBalanceByAddress";
pub const STATUS_CONFIG_UPDATE_TOPIC: &'static str = "status_config_update";
pub const STATUS_CONFIG_GET_LIST_TOPIC: &'static str = "status_config_getList";
pub const STATUS_STATUS_GET_USER_CONFIG_TOPIC: &'static str = "status_status_getUserConfig";
pub const STATUS_STATUS_ADD_TOPIC: &'static str = "status_status_add";
pub const STATUS_STATUS_REMOVE_TOPIC: &'static str = "status_status_remove";
pub const STATUS_STATUS_GET_BY_USER_ID_TOPIC: &'static str = "status_status_getByUserId";
pub const TRANSACTIONS_BALANCE_GET_BALANCE_BY_USER_ID_TOPIC: &'static str = "transactions_balance_getBalanceByUserId";
pub const TRANSACTIONS_BALANCE_GET_BALANCES_BY_USER_ID_TOPIC: &'static str = "transactions_balance_getBalancesByUserId";
pub const TRANSACTIONS_BALANCE_GET_BALANCES_BY_USER_ID_AS_ARRAY_TOPIC: &'static str =
    "transactions_balance_getBalancesByUserIdAsArray";
pub const TRANSACTIONS_BALANCE_GET_BALANCES_IN_USD_TOPIC: &'static str = "transactions_balance_getBalancesInUsd";
pub const TRANSACTIONS_BALANCE_INCREASE_BALANCE_BY_USER_ID_TOPIC: &'static str =
    "transactions_balance_increaseBalanceByUserId";
pub const TRANSACTIONS_BALANCE_DECREASE_BALANCE_BY_USER_ID_TOPIC: &'static str =
    "transactions_balance_decreaseBalanceByUserId";
pub const TRANSACTIONS_BALANCE_PURCHASE_PRODUCT_TOPIC: &'static str = "transactions_balance_purchaseProduct";
pub const TRANSACTIONS_BALANCE_GET_ALL_TOPIC: &'static str = "transactions_balance_getAll";
pub const TRANSACTIONS_BALANCE_GET_ALL_MY_TOPIC: &'static str = "transactions_balance_getAllMy";
pub const TRANSACTIONS_BALANCE_GET_MY_NUM_OF_TRANSACTIONS_TOPIC: &'static str =
    "transactions_balance_getMyNumOfTransactions";
pub const TRANSACTIONS_BALANCE_GET_ALL_CREDIT_TRANSACTION_LIST_TOPIC: &'static str =
    "transactions_balance_getAllCreditTransactionList";
pub const TRANSACTIONS_BALANCE_GET_USER_BALANCES_TOPIC: &'static str = "transactions_balance_getUserBalances";
pub const TRANSACTIONS_BALANCE_GET_WITH_FILTER_TOPIC: &'static str = "transactions_balance_getWithFilter";
pub const TRANSACTIONS_BALANCE_GET_NUM_OF_TRANSACTIONS_TOPIC: &'static str =
    "transactions_balance_getNumOfTransactions";
pub const TRANSACTIONS_BALANCE_GET_NUM_WITH_FILTER_TOPIC: &'static str = "transactions_balance_getNumWithFilter";
pub const TRANSACTIONS_BALANCE_UPDATE_TRANSACTION_HASH_AND_BLOCK_ID_TOPIC: &'static str =
    "transactions_balance_updateTransactionHashAndBlockId";
pub const TRANSACTIONS_BALANCE_GET_LIST_WITH_PAGINATION_TOPIC: &'static str =
    "transactions_balance_getListWithPagination";
pub const TRANSACTIONS_BALANCE_DELETE_BALANCE_BY_USER_ID_TOPIC: &'static str =
    "transactions_balance_deleteBalanceByUserId";
pub const TRANSACTIONS_REGISTRY_PRODUCT_ADD_PRODUCT_TOPIC: &'static str = "transactions_registryProduct_addProduct";
pub const TRANSACTIONS_REGISTRY_PRODUCT_ADD_PRODUCT_HASH_TOPIC: &'static str =
    "transactions_registryProduct_addProductHash";
pub const TRANSACTIONS_REGISTRY_PRODUCT_GET_USER_ITEMS_TOPIC: &'static str =
    "transactions_registryProduct_getUserItems";
pub const TRANSACTIONS_REGISTRY_PRODUCT_GET_PERSONAL_TYPES_BY_USER_TOPIC: &'static str =
    "transactions_registryProduct_getPersonalTypesByUser";
pub const TRANSACTIONS_REGISTRY_PRODUCT_TRANSFER_PRODUCT_TOPIC: &'static str =
    "transactions_registryProduct_transferProduct";
pub const TRANSACTIONS_REGISTRY_PRODUCT_DELETE_PRODUCT_BY_USER_ID_TOPIC: &'static str =
    "transactions_registryProduct_deleteProductByUserId";
pub const USER_PROFILE_ASSIGN_STATUS_TOPIC: &'static str = "user_profile_assignStatus";
pub const USER_PROFILE_CHANGE_NICKNAME_TOPIC: &'static str = "user_profile_changeNickname";
pub const USER_PROFILE_SET_NICKNAME_TOPIC: &'static str = "user_profile_setNickname";
pub const USER_PROFILE_GET_PROFILE_TOPIC: &'static str = "user_profile_getProfile";
pub const USER_PROFILE_GET_MY_PROFILE_TOPIC: &'static str = "user_profile_getMyProfile";
pub const USER_PROFILE_GET_ALL_PROFILES_TOPIC: &'static str = "user_profile_getAllProfiles";
pub const USER_PROFILE_UNSET_NOTICE_TOPIC: &'static str = "user_profile_unsetNotice";
pub const USER_PROFILE_SET_SUSPICIOUS_TOPIC: &'static str = "user_profile_setSuspicious";
pub const USER_PROFILE_UNSET_SUSPICIOUS_TOPIC: &'static str = "user_profile_unsetSuspicious";
pub const USER_PROFILE_BAN_PROFILE_TOPIC: &'static str = "user_profile_banProfile";
pub const USER_PROFILE_UN_BAN_TOPIC: &'static str = "user_profile_unBan";
pub const USER_PROFILE_SET_MAILING_SUBSCRIPTION_STATUS_TOPIC: &'static str =
    "user_profile_setMailingSubscriptionStatus";
pub const USER_PROFILE_GET_ALL_USERS_SUBSCRIBED_TO_MAILING_TOPIC: &'static str =
    "user_profile_getAllUsersSubscribedToMailing";
pub const USER_PROFILE_GET_BAN_INFO_TOPIC: &'static str = "user_profile_getBanInfo";
pub const USER_PROFILE_GET_MY_BAN_INFO_TOPIC: &'static str = "user_profile_getMyBanInfo";
pub const USER_PROFILE_GET_NUM_TOPIC: &'static str = "user_profile_getNum";
pub const USER_PROFILE_GET_NICKNAME_CHANGE_PRICE_TOPIC: &'static str = "user_profile_getNicknameChangePrice";
pub const USER_PROFILE_GET_USER_BY_PARAMS_TOPIC: &'static str = "user_profile_getUserByParams";
pub const USER_PROFILE_GET_MY_NICKNAME_TOPIC: &'static str = "user_profile_getMyNickname";
pub const USER_PROFILE_GET_USER_NICKNAME_TOPIC: &'static str = "user_profile_getUserNickname";
pub const USER_PROFILE_GET_NICKNAMES_BY_USER_IDS_TOPIC: &'static str = "user_profile_getNicknamesByUserIds";
pub const USER_PROFILE_DELETE_USER_TOPIC: &'static str = "user_profile_deleteUser";
pub const USER_PROFILE_SET_NOTICE_TOPIC: &'static str = "user_profile_setNotice";
pub const USER_PROFILE_GET_MY_PROFILE_FOR_BYTES_TOPIC: &'static str = "user_profile_getMyProfileForBytes";
pub const USER_PROFILE_CHANGE_EMAIL_TOPIC: &'static str = "user_profile_changeEmail";
pub const USER_PROFILE_GET_TEST_DATA_TOPIC: &'static str = "user_profile_getTestData";
pub const USER_PROFILE_IS_EMAIL_EXISTS_TOPIC: &'static str = "user_profile_isEmailExists";
pub const USER_PROFILE_GET_USER_ID_BY_EMAIL_TOPIC: &'static str = "user_profile_getUserIdByEmail";
pub const USER_PROFILE_DELETE_USER_RECORD_TOPIC: &'static str = "user_profile_deleteUserRecord";
pub const USER_STATUS_DELETE_TOPIC: &'static str = "user_status_delete";
pub const USER_STATUS_GET_TOPIC: &'static str = "user_status_get";
pub const WITHDRAW_WITHDRAW_WITHDRAW_TOPIC: &'static str = "withdraw_withdraw_withdraw";
pub const CRON_BROADCAST_TOPIC: &'static str = "cron_broadcast";
pub const TECHNICAL_NOTIFICATIONS_TOPIC: &'static str = "technical_notifications";
pub const SEND_USER_NOTIFICATIONS_TOPIC: &'static str = "send_user_notifications";
pub const READ_USER_NOTIFICATION_TOPIC: &'static str = "read_user_notification";
pub const USER_ONLINE_TOPIC: &'static str = "user_online";
pub const OPTIONS_UPDATE_TOPIC: &'static str = "subscribe_to_options_update";
pub const ADMIN_ACTIONS_BROADCAST: &'static str = "admin_actions_broadcast";
pub const ADMIN_ACTIONS_RESPONSE_BROADCAST: &'static str = "admin_actions_response_broadcast";
pub const PURCHASE_ORCHESTRATOR_TOPIC: &'static str = "subscribe_to_options_update";
pub const BALANCE_INCREASE_PROCESS_TOPIC: &'static str = "balance_increase_process_topic";
pub const BALANCE_DECREASE_ADAPTER_PROCESS_TOPIC: &'static str = "credit_hard_currency";
pub const BALANCE_INCREASE_ADAPTER_PROCESS_TOPIC: &'static str = "debit_hard_currency";
pub const BALANCE_ADAPTER_INCREASED_TOPIC: &'static str = "balance_adapter_increased";
pub const BALANCE_ADAPTER_DECREASED_TOPIC: &'static str = "balance_adapter_decreased";
pub const BALANCE_INCREASED_TOPIC: &'static str = "balance_increased_topic";
pub const FACTORY_PROCESS_TOPIC: &'static str = "factory_process_topic";
pub const FACTORY_PROCESSED_TOPIC: &'static str = "factory_processed_topic";
pub const BALANCE_DECREASE_PROCESS_TOPIC: &'static str = "balance_decrease_process_topic";
pub const BALANCE_DECREASE_PROCESSED_TOPIC: &'static str = "balance_decrease_processed_topic";
pub const REFERRAL_FROM_ADAPTER_PROCESSED_TOPIC: &'static str = "referral_from_adapter_processed_topic";
pub const REFERRAL_FROM_TRANSACTION_PROCESSED_TOPIC: &'static str = "referral_from_transaction_processed_topic";
pub const PAY_REWARD_PROCESS_TOPIC: &'static str = "pay_reward_process_topic";
pub const REWARD_REFERRAL_FROM_ADAPTER_PROCESSED_TOPIC: &'static str = "reward_referral_from_adapter_processed_topic";
pub const REWARD_REFERRAL_FROM_TRANSACTION_PROCESSED_TOPIC: &'static str =
    "reward_referral_from_transactions_processed_topic";
pub const RESPONSE_FROM_ORCHESTRATOR_TOPIC: &'static str = "response_from_orchestrator_topic";
pub const PURCHASE_TRANSACTION_RESPONSE_TOPIC: &'static str = "purchase_transaction_response_topic";
pub const PURCHASE_ADAPTER_RESPONSE_TOPIC: &'static str = "purchase_adapter_response_topic";
pub const CHANGE_NICKNAME_ORCHESTRATOR_TOPIC: &'static str = "change_nickname_orchestrator_topic";
pub const GOOGLE_PLAY_PURCHASE_TOPIC: &'static str = "google_play_purchase";
pub const EXCHANGE_RATES_TOPIC: &'static str = "exchange_rates";
pub const NEW_USER_BROADCAST_TOPIC: &'static str = "new_user_broadcast";
pub const DELETE_USER_BROADCAST_TOPIC: &'static str = "delete_user_broadcast";
pub const RESPONSE_CHANGE_NICKNAME_ORCHESTRATOR_TOPIC: &'static str = "response_change_nickname_orchestrator_topic";
pub const REALIS_ADAPTER_TOPIC: &'static str = "realis_adapter";
pub const LOBBY_TO_REALIS_GET_NFT_HASH_TOPIC: &'static str = "lobby_to_realis_get_nft_hash";
pub const ORCHESTRATOR_TO_REALIS_GET_NFT_HASH_TOPIC: &'static str = "orchestrator_to_realis_get_nft_hash";
pub const SUBSCRIBE_TO_CHANGE_NICKNAME_TOPIC_BC: &'static str = "nickname_change_bc_topic";
pub const ADD_NFT_ITEM_TOPIC: &'static str = "add_nft_item";
pub const CONFIRM_EMAIL_TOPIC: &'static str = "confirm_email";
pub const GET_USER_INFO_BY_TOKEN_TOPIC: &'static str = "get_user_info_by_token";
pub const OLD_USERS_TO_GAME_TOPIC: &'static str = "old_users_to_game";
pub const OLD_USERS_TO_GAME_CATS_TOPIC: &'static str = "old_users_to_game_cats";
pub const OLD_USERS_TO_GAME_DRAGONS_TOPIC: &'static str = "old_users_to_game_dragons";
pub const NEW_USERS_TO_GAME_TOPIC: &'static str = "new_users_to_game";
pub const NEW_USERS_TO_GAME_CATS_TOPIC: &'static str = "new_users_to_game_cats";
pub const NEW_USERS_TO_GAME_DRAGONS_TOPIC: &'static str = "new_users_to_game_dragons";
pub const DELETE_USER_TOPIC: &'static str = "delete_user";
pub const SUBSCRIBE_TO_CHANGE_NICKNAME_TOPIC_TSX: &'static str = "nickname_change_topic_tsx";
pub const SUBSCRIBE_TO_GET_USER_ID_AND_PRODUCT_ID_PF: &'static str = "get_userId_and_productId_pf";
pub const SUBSCRIBE_TO_GET_PRODUCT_ID_AND_HASH_ID_PF: &'static str = "get_productId_and_hashId_pf";
pub const SEND_PRODUCT_TO_BURN: &'static str = "send_product_to_burn";
pub const SEND_USER_ID_AND_PRODUCT_ID_PF: &'static str = "send_userId_and_productId_pf";
pub const OPEN_LOOTBOX_TOPIC: &'static str = "open_lootbox_topic";
pub const GET_ITEMS_FROM_LOOTBOX_TOPIC: &'static str = "get_items_from_lootbox_topic";
pub const ADD_HERO_TOPIC: &'static str = "add_hero_topic";
pub const ADD_ITEM_TOPIC: &'static str = "add_item_topic";
pub const SEARCH_GAME_TOPIC: &'static str = "search_game";
pub const START_GAME_TOPIC: &'static str = "start_game";
pub const STATISTIC_BROADCAST_TOPIC: &'static str = "statistic_broadcast";
pub const CHECK_IN_BROADCAST_TOPIC: &'static str = "check_in_broadcast";
pub const GATEWAY_NOTIFICATION_TOPIC: &'static str = "gateway-notification";
