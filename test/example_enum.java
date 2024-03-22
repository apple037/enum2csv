package com.gash.application.itemsplatform.exception;

import com.gash.common.exception.ErrorLevel;
import lombok.Getter;

@Getter
public enum ItemsPlatformExceptionEnum {
        // 1000xxx 通用
        DATA_NOT_FOUND("10000001", "data not found.", "查無資料", ErrorLevel.WARN),
        DATA_INSERT_FAIL("10000002", "data insert fail.", "新增資料失敗", ErrorLevel.WARN),
        DATA_UPDATE_FAIL("1000003", "data update fail.", "更新資料失敗", ErrorLevel.WARN),
        DATA_DELETE_FAIL("10000004", "data delete fail.", "刪除資料失敗", ErrorLevel.WARN),
        DATA_ALREADY_EXIST("10000005", "data already exist.", "資料已存在", ErrorLevel.WARN),
        ACTIVATION_DATE_ERROR("10000006", "activation date error.", "啟用日期錯誤", ErrorLevel.WARN),
        DEACTIVATION_DATE_ERROR("10000007", "deactivation date error.", "停用日期錯誤", ErrorLevel.WARN),
        MISSING_PARAMETER("10000008", "missing parameter.", "缺少參數", ErrorLevel.WARN),
        SECRET_GENERATION_ERROR("10000009", "secret generation error.", "金鑰產生錯誤",
                        ErrorLevel.WARN),

        // 1001xxx 物件標籤
        ITEM_TAG_CODE_DUPLICATE("10000010", "item tag code duplicate.", "物件標籤代碼重複",
                        ErrorLevel.WARN),
        PARENT_ITEM_TAG_NOT_FOUND("10000011", "parent item tag not found.", "父物件標籤不存在",
                        ErrorLevel.WARN),
        ITEM_TAG_NOT_FOUND("10000012", "item tag not found.", "物件標籤不存在", ErrorLevel.WARN),
        ITEM_TAG_IS_PARENT("10000013", "item tag is parent.", "物件標籤為父標籤", ErrorLevel.WARN),
        ITEM_TAG_PARENT_IS_SELF("10000014", "item tag parent is self.", "父物件標籤為自己",
                        ErrorLevel.WARN),

        // 1002xxx 物件群組
        ITEM_GROUP_CODE_NOT_FOUND("10000015", "item group code not found.", "物件群組代碼不存在",
                        ErrorLevel.WARN),
        ITEM_GROUP_CODE_DUPLICATE("10000016", "item group code duplicate.", "物件群組代碼重複",
                        ErrorLevel.WARN),
        ITEM_GROUP_IS_PARENT("10000017", "item group is parent.", "物件群組為父群組", ErrorLevel.WARN),

        // 1003xxx 物件類型
        ITEM_TYPE_DUPLICATE("10000018", "item type duplicate.", "物件類型重複", ErrorLevel.WARN),
        ITEM_TYPE_NOT_FOUND("10000019", "item type not found.", "物件類型不存在", ErrorLevel.WARN),
        ITEM_TYPE_IS_DELETED("10000020", "item type is deleted.", "物件類型已刪除", ErrorLevel.WARN),

        // 1004xxx 物件供應商
        CONTENT_PROVIDER_CODE_DUPLICATE(
                        "10000021",
                        "content provider code duplicate.",
                        "內容服務商代碼重複",
                        ErrorLevel.WARN),
        CONTENT_PROVIDER_CODE_NOT_EXIST(
                        "10000022",
                        "content provider code not exist.",
                        "內容服務商代碼不存在",
                        ErrorLevel.WARN),
        CONTENT_SERVER_CONNECTION_EXIST(
                        "10000023",
                        "content server connection exist.",
                        "內容服務商連線已存在",
                        ErrorLevel.WARN),
        CONTENT_SERVER_CONNECTION_NOT_EXIST(
                        "10000024",
                        "content server connection not exist.",
                        "內容服務商連線不存在",
                        ErrorLevel.WARN),
        CONTENT_SERVER_CODE_DUPLICATE(
                        "10000025",
                        "content server code duplicate.",
                        "內容服務商伺服器代碼重複",
                        ErrorLevel.WARN),
        CONTENT_SERVER_CODE_NOT_EXIST(
                        "10000026",
                        "content server code not exist.",
                        "內容服務商伺服器代碼不存在",
                        ErrorLevel.WARN),
        CONTENT_PROVIDER_DELETED(
                        "10000027",
                        "content provider deleted.",
                        "內容服務商已刪除",
                        ErrorLevel.WARN),
        CONTENT_PROVIDER_INACTIVE(
                        "10000028",
                        "content provider inactive.",
                        "內容服務商未啟用",
                        ErrorLevel.WARN),
        CONTENT_PROVIDER_EXPIRED(
                        "10000029",
                        "content provider expired.",
                        "內容服務商已過期",
                        ErrorLevel.WARN),
                        ;

        private final String code;
        private final String message;
        private final String description;
        private final ErrorLevel errorLevel;

        ItemsPlatformExceptionEnum(
                        String code, String message, String description, ErrorLevel errorLevel) {
                this.code = code;
                this.message = message;
                this.description = description;
                this.errorLevel = errorLevel;
        }
}
