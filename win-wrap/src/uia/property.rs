/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

pub use windows::Win32::UI::Accessibility::{
    UIA_AcceleratorKeyPropertyId, UIA_AccessKeyPropertyId,
    UIA_AnnotationAnnotationTypeIdPropertyId, UIA_AnnotationAnnotationTypeNamePropertyId,
    UIA_AnnotationAuthorPropertyId, UIA_AnnotationDateTimePropertyId,
    UIA_AnnotationObjectsPropertyId, UIA_AnnotationTargetPropertyId, UIA_AnnotationTypesPropertyId,
    UIA_AriaPropertiesPropertyId, UIA_AriaRolePropertyId, UIA_AutomationIdPropertyId,
    UIA_BoundingRectanglePropertyId, UIA_CenterPointPropertyId, UIA_ClassNamePropertyId,
    UIA_ClickablePointPropertyId, UIA_ControlTypePropertyId, UIA_ControllerForPropertyId,
    UIA_CulturePropertyId, UIA_DescribedByPropertyId, UIA_DockDockPositionPropertyId,
    UIA_DragDropEffectPropertyId, UIA_DragDropEffectsPropertyId, UIA_DragGrabbedItemsPropertyId,
    UIA_DragIsGrabbedPropertyId, UIA_DropTargetDropTargetEffectPropertyId,
    UIA_DropTargetDropTargetEffectsPropertyId, UIA_ExpandCollapseExpandCollapseStatePropertyId,
    UIA_FillColorPropertyId, UIA_FillTypePropertyId, UIA_FlowsFromPropertyId,
    UIA_FlowsToPropertyId, UIA_FrameworkIdPropertyId, UIA_FullDescriptionPropertyId,
    UIA_GridColumnCountPropertyId, UIA_GridItemColumnPropertyId, UIA_GridItemColumnSpanPropertyId,
    UIA_GridItemContainingGridPropertyId, UIA_GridItemRowPropertyId, UIA_GridItemRowSpanPropertyId,
    UIA_GridRowCountPropertyId, UIA_HasKeyboardFocusPropertyId, UIA_HeadingLevelPropertyId,
    UIA_HelpTextPropertyId, UIA_IsAnnotationPatternAvailablePropertyId,
    UIA_IsContentElementPropertyId, UIA_IsControlElementPropertyId,
    UIA_IsCustomNavigationPatternAvailablePropertyId, UIA_IsDataValidForFormPropertyId,
    UIA_IsDialogPropertyId, UIA_IsDockPatternAvailablePropertyId,
    UIA_IsDragPatternAvailablePropertyId, UIA_IsDropTargetPatternAvailablePropertyId,
    UIA_IsEnabledPropertyId, UIA_IsExpandCollapsePatternAvailablePropertyId,
    UIA_IsGridItemPatternAvailablePropertyId, UIA_IsGridPatternAvailablePropertyId,
    UIA_IsInvokePatternAvailablePropertyId, UIA_IsItemContainerPatternAvailablePropertyId,
    UIA_IsKeyboardFocusablePropertyId, UIA_IsLegacyIAccessiblePatternAvailablePropertyId,
    UIA_IsMultipleViewPatternAvailablePropertyId, UIA_IsObjectModelPatternAvailablePropertyId,
    UIA_IsOffscreenPropertyId, UIA_IsPasswordPropertyId, UIA_IsPeripheralPropertyId,
    UIA_IsRangeValuePatternAvailablePropertyId, UIA_IsRequiredForFormPropertyId,
    UIA_IsScrollItemPatternAvailablePropertyId, UIA_IsScrollPatternAvailablePropertyId,
    UIA_IsSelectionItemPatternAvailablePropertyId, UIA_IsSelectionPattern2AvailablePropertyId,
    UIA_IsSelectionPatternAvailablePropertyId, UIA_IsSpreadsheetItemPatternAvailablePropertyId,
    UIA_IsSpreadsheetPatternAvailablePropertyId, UIA_IsStylesPatternAvailablePropertyId,
    UIA_IsSynchronizedInputPatternAvailablePropertyId, UIA_IsTableItemPatternAvailablePropertyId,
    UIA_IsTablePatternAvailablePropertyId, UIA_IsTextChildPatternAvailablePropertyId,
    UIA_IsTextEditPatternAvailablePropertyId, UIA_IsTextPattern2AvailablePropertyId,
    UIA_IsTextPatternAvailablePropertyId, UIA_IsTogglePatternAvailablePropertyId,
    UIA_IsTransformPattern2AvailablePropertyId, UIA_IsTransformPatternAvailablePropertyId,
    UIA_IsValuePatternAvailablePropertyId, UIA_IsVirtualizedItemPatternAvailablePropertyId,
    UIA_IsWindowPatternAvailablePropertyId, UIA_ItemStatusPropertyId, UIA_ItemTypePropertyId,
    UIA_LabeledByPropertyId, UIA_LandmarkTypePropertyId, UIA_LegacyIAccessibleChildIdPropertyId,
    UIA_LegacyIAccessibleDefaultActionPropertyId, UIA_LegacyIAccessibleDescriptionPropertyId,
    UIA_LegacyIAccessibleHelpPropertyId, UIA_LegacyIAccessibleKeyboardShortcutPropertyId,
    UIA_LegacyIAccessibleNamePropertyId, UIA_LegacyIAccessibleRolePropertyId,
    UIA_LegacyIAccessibleSelectionPropertyId, UIA_LegacyIAccessibleStatePropertyId,
    UIA_LegacyIAccessibleValuePropertyId, UIA_LevelPropertyId, UIA_LiveSettingPropertyId,
    UIA_LocalizedControlTypePropertyId, UIA_LocalizedLandmarkTypePropertyId,
    UIA_MultipleViewCurrentViewPropertyId, UIA_MultipleViewSupportedViewsPropertyId,
    UIA_NamePropertyId, UIA_NativeWindowHandlePropertyId, UIA_OptimizeForVisualContentPropertyId,
    UIA_OrientationPropertyId, UIA_OutlineColorPropertyId, UIA_OutlineThicknessPropertyId,
    UIA_PositionInSetPropertyId, UIA_ProcessIdPropertyId, UIA_ProviderDescriptionPropertyId,
    UIA_RangeValueIsReadOnlyPropertyId, UIA_RangeValueLargeChangePropertyId,
    UIA_RangeValueMaximumPropertyId, UIA_RangeValueMinimumPropertyId,
    UIA_RangeValueSmallChangePropertyId, UIA_RangeValueValuePropertyId, UIA_RotationPropertyId,
    UIA_RuntimeIdPropertyId, UIA_ScrollHorizontalScrollPercentPropertyId,
    UIA_ScrollHorizontalViewSizePropertyId, UIA_ScrollHorizontallyScrollablePropertyId,
    UIA_ScrollVerticalScrollPercentPropertyId, UIA_ScrollVerticalViewSizePropertyId,
    UIA_ScrollVerticallyScrollablePropertyId, UIA_Selection2CurrentSelectedItemPropertyId,
    UIA_Selection2FirstSelectedItemPropertyId, UIA_Selection2ItemCountPropertyId,
    UIA_Selection2LastSelectedItemPropertyId, UIA_SelectionCanSelectMultiplePropertyId,
    UIA_SelectionIsSelectionRequiredPropertyId, UIA_SelectionItemIsSelectedPropertyId,
    UIA_SelectionItemSelectionContainerPropertyId, UIA_SelectionSelectionPropertyId,
    UIA_SizeOfSetPropertyId, UIA_SizePropertyId, UIA_SpreadsheetItemAnnotationObjectsPropertyId,
    UIA_SpreadsheetItemAnnotationTypesPropertyId, UIA_SpreadsheetItemFormulaPropertyId,
    UIA_StylesExtendedPropertiesPropertyId, UIA_StylesFillColorPropertyId,
    UIA_StylesFillPatternColorPropertyId, UIA_StylesFillPatternStylePropertyId,
    UIA_StylesShapePropertyId, UIA_StylesStyleIdPropertyId, UIA_StylesStyleNamePropertyId,
    UIA_TableColumnHeadersPropertyId, UIA_TableItemColumnHeaderItemsPropertyId,
    UIA_TableItemRowHeaderItemsPropertyId, UIA_TableRowHeadersPropertyId,
    UIA_TableRowOrColumnMajorPropertyId, UIA_ToggleToggleStatePropertyId,
    UIA_Transform2CanZoomPropertyId, UIA_Transform2ZoomLevelPropertyId,
    UIA_Transform2ZoomMaximumPropertyId, UIA_Transform2ZoomMinimumPropertyId,
    UIA_TransformCanMovePropertyId, UIA_TransformCanResizePropertyId,
    UIA_TransformCanRotatePropertyId, UIA_ValueIsReadOnlyPropertyId, UIA_ValueValuePropertyId,
    UIA_VisualEffectsPropertyId, UIA_WindowCanMaximizePropertyId, UIA_WindowCanMinimizePropertyId,
    UIA_WindowIsModalPropertyId, UIA_WindowIsTopmostPropertyId,
    UIA_WindowWindowInteractionStatePropertyId, UIA_WindowWindowVisualStatePropertyId,
    UIA_PROPERTY_ID,
};

/// UIA元素属性枚举。
pub enum UiaPropertyId {
    AcceleratorKey,
    AccessKey,
    AnnotationAnnotationTypeId,
    AnnotationAnnotationTypeName,
    AnnotationAuthor,
    AnnotationDateTime,
    AnnotationObjects,
    AnnotationTarget,
    AnnotationTypes,
    AriaProperties,
    AriaRole,
    AutomationId,
    BoundingRectangle,
    CenterPoint,
    ClassName,
    ClickablePoint,
    ControlType,
    ControllerFor,
    Culture,
    DescribedBy,
    DockDockPosition,
    DragDropEffect,
    DragDropEffects,
    DragGrabbedItems,
    DragIsGrabbed,
    DropTargetDropTargetEffect,
    DropTargetDropTargetEffects,
    ExpandCollapseExpandCollapseState,
    FillColor,
    FillType,
    FlowsFrom,
    FlowsTo,
    FrameworkId,
    FullDescription,
    GridColumnCount,
    GridItemColumn,
    GridItemColumnSpan,
    GridItemContainingGrid,
    GridItemRow,
    GridItemRowSpan,
    GridRowCount,
    HasKeyboardFocus,
    HeadingLevel,
    HelpText,
    IsAnnotationPatternAvailable,
    IsContentElement,
    IsControlElement,
    IsCustomNavigationPatternAvailable,
    IsDataValidForForm,
    IsDialog,
    IsDockPatternAvailable,
    IsDragPatternAvailable,
    IsDropTargetPatternAvailable,
    IsEnabled,
    IsExpandCollapsePatternAvailable,
    IsGridItemPatternAvailable,
    IsGridPatternAvailable,
    IsInvokePatternAvailable,
    IsItemContainerPatternAvailable,
    IsKeyboardFocusable,
    IsLegacyIAccessiblePatternAvailable,
    IsMultipleViewPatternAvailable,
    IsObjectModelPatternAvailable,
    IsOffscreen,
    IsPassword,
    IsPeripheral,
    IsRangeValuePatternAvailable,
    IsRequiredForForm,
    IsScrollItemPatternAvailable,
    IsScrollPatternAvailable,
    IsSelectionItemPatternAvailable,
    IsSelectionPattern2Available,
    IsSelectionPatternAvailable,
    IsSpreadsheetItemPatternAvailable,
    IsSpreadsheetPatternAvailable,
    IsStylesPatternAvailable,
    IsSynchronizedInputPatternAvailable,
    IsTableItemPatternAvailable,
    IsTablePatternAvailable,
    IsTextChildPatternAvailable,
    IsTextEditPatternAvailable,
    IsTextPattern2Available,
    IsTextPatternAvailable,
    IsTogglePatternAvailable,
    IsTransformPattern2Available,
    IsTransformPatternAvailable,
    IsValuePatternAvailable,
    IsVirtualizedItemPatternAvailable,
    IsWindowPatternAvailable,
    ItemStatus,
    ItemType,
    LabeledBy,
    LandmarkType,
    LegacyIAccessibleChildId,
    LegacyIAccessibleDefaultAction,
    LegacyIAccessibleDescription,
    LegacyIAccessibleHelp,
    LegacyIAccessibleKeyboardShortcut,
    LegacyIAccessibleName,
    LegacyIAccessibleRole,
    LegacyIAccessibleSelection,
    LegacyIAccessibleState,
    LegacyIAccessibleValue,
    Level,
    LiveSetting,
    LocalizedControlType,
    LocalizedLandmarkType,
    MultipleViewCurrentView,
    MultipleViewSupportedViews,
    Name,
    NativeWindowHandle,
    OptimizeForVisualContent,
    Orientation,
    OutlineColor,
    OutlineThickness,
    PositionInSet,
    ProcessId,
    ProviderDescription,
    RangeValueIsReadOnly,
    RangeValueLargeChange,
    RangeValueMaximum,
    RangeValueMinimum,
    RangeValueSmallChange,
    RangeValueValue,
    Rotation,
    RuntimeId,
    ScrollHorizontalScrollPercent,
    ScrollHorizontalViewSize,
    ScrollHorizontallyScrollable,
    ScrollVerticalScrollPercent,
    ScrollVerticalViewSize,
    ScrollVerticallyScrollable,
    Selection2CurrentSelectedItem,
    Selection2FirstSelectedItem,
    Selection2ItemCount,
    Selection2LastSelectedItem,
    SelectionCanSelectMultiple,
    SelectionIsSelectionRequired,
    SelectionItemIsSelected,
    SelectionItemSelectionContainer,
    SelectionSelection,
    SizeOfSet,
    Size,
    SpreadsheetItemAnnotationObjects,
    SpreadsheetItemAnnotationTypes,
    SpreadsheetItemFormula,
    StylesExtendedProperties,
    StylesFillColor,
    StylesFillPatternColor,
    StylesFillPatternStyle,
    StylesShape,
    StylesStyleId,
    StylesStyleName,
    TableColumnHeaders,
    TableItemColumnHeaderItems,
    TableItemRowHeaderItems,
    TableRowHeaders,
    TableRowOrColumnMajor,
    ToggleToggleState,
    Transform2CanZoom,
    Transform2ZoomLevel,
    Transform2ZoomMaximum,
    Transform2ZoomMinimum,
    TransformCanMove,
    TransformCanResize,
    TransformCanRotate,
    ValueIsReadOnly,
    ValueValue,
    VisualEffects,
    WindowCanMaximize,
    WindowCanMinimize,
    WindowIsModal,
    WindowIsTopmost,
    WindowWindowInteractionState,
    WindowWindowVisualState,
}

impl Into<UIA_PROPERTY_ID> for UiaPropertyId {
    fn into(self) -> UIA_PROPERTY_ID {
        match self {
            Self::AcceleratorKey => UIA_AcceleratorKeyPropertyId,
            Self::AccessKey => UIA_AccessKeyPropertyId,
            Self::AnnotationAnnotationTypeId => UIA_AnnotationAnnotationTypeIdPropertyId,
            Self::AnnotationAnnotationTypeName => UIA_AnnotationAnnotationTypeNamePropertyId,
            Self::AnnotationAuthor => UIA_AnnotationAuthorPropertyId,
            Self::AnnotationDateTime => UIA_AnnotationDateTimePropertyId,
            Self::AnnotationObjects => UIA_AnnotationObjectsPropertyId,
            Self::AnnotationTarget => UIA_AnnotationTargetPropertyId,
            Self::AnnotationTypes => UIA_AnnotationTypesPropertyId,
            Self::AriaProperties => UIA_AriaPropertiesPropertyId,
            Self::AriaRole => UIA_AriaRolePropertyId,
            Self::AutomationId => UIA_AutomationIdPropertyId,
            Self::BoundingRectangle => UIA_BoundingRectanglePropertyId,
            Self::CenterPoint => UIA_CenterPointPropertyId,
            Self::ClassName => UIA_ClassNamePropertyId,
            Self::ClickablePoint => UIA_ClickablePointPropertyId,
            Self::ControlType => UIA_ControlTypePropertyId,
            Self::ControllerFor => UIA_ControllerForPropertyId,
            Self::Culture => UIA_CulturePropertyId,
            Self::DescribedBy => UIA_DescribedByPropertyId,
            Self::DockDockPosition => UIA_DockDockPositionPropertyId,
            Self::DragDropEffect => UIA_DragDropEffectPropertyId,
            Self::DragDropEffects => UIA_DragDropEffectsPropertyId,
            Self::DragGrabbedItems => UIA_DragGrabbedItemsPropertyId,
            Self::DragIsGrabbed => UIA_DragIsGrabbedPropertyId,
            Self::DropTargetDropTargetEffect => UIA_DropTargetDropTargetEffectPropertyId,
            Self::DropTargetDropTargetEffects => UIA_DropTargetDropTargetEffectsPropertyId,
            Self::ExpandCollapseExpandCollapseState => UIA_ExpandCollapseExpandCollapseStatePropertyId,
            Self::FillColor => UIA_FillColorPropertyId,
            Self::FillType => UIA_FillTypePropertyId,
            Self::FlowsFrom => UIA_FlowsFromPropertyId,
            Self::FlowsTo => UIA_FlowsToPropertyId,
            Self::FrameworkId => UIA_FrameworkIdPropertyId,
            Self::FullDescription => UIA_FullDescriptionPropertyId,
            Self::GridColumnCount => UIA_GridColumnCountPropertyId,
            Self::GridItemColumn => UIA_GridItemColumnPropertyId,
            Self::GridItemColumnSpan => UIA_GridItemColumnSpanPropertyId,
            Self::GridItemContainingGrid => UIA_GridItemContainingGridPropertyId,
            Self::GridItemRow => UIA_GridItemRowPropertyId,
            Self::GridItemRowSpan => UIA_GridItemRowSpanPropertyId,
            Self::GridRowCount => UIA_GridRowCountPropertyId,
            Self::HasKeyboardFocus => UIA_HasKeyboardFocusPropertyId,
            Self::HeadingLevel => UIA_HeadingLevelPropertyId,
            Self::HelpText => UIA_HelpTextPropertyId,
            Self::IsAnnotationPatternAvailable => UIA_IsAnnotationPatternAvailablePropertyId,
            Self::IsContentElement => UIA_IsContentElementPropertyId,
            Self::IsControlElement => UIA_IsControlElementPropertyId,
            Self::IsCustomNavigationPatternAvailable => UIA_IsCustomNavigationPatternAvailablePropertyId,
            Self::IsDataValidForForm => UIA_IsDataValidForFormPropertyId,
            Self::IsDialog => UIA_IsDialogPropertyId,
            Self::IsDockPatternAvailable => UIA_IsDockPatternAvailablePropertyId,
            Self::IsDragPatternAvailable => UIA_IsDragPatternAvailablePropertyId,
            Self::IsDropTargetPatternAvailable => UIA_IsDropTargetPatternAvailablePropertyId,
            Self::IsEnabled => UIA_IsEnabledPropertyId,
            Self::IsExpandCollapsePatternAvailable => UIA_IsExpandCollapsePatternAvailablePropertyId,
            Self::IsGridItemPatternAvailable => UIA_IsGridItemPatternAvailablePropertyId,
            Self::IsGridPatternAvailable => UIA_IsGridPatternAvailablePropertyId,
            Self::IsInvokePatternAvailable => UIA_IsInvokePatternAvailablePropertyId,
            Self::IsItemContainerPatternAvailable => UIA_IsItemContainerPatternAvailablePropertyId,
            Self::IsKeyboardFocusable => UIA_IsKeyboardFocusablePropertyId,
            Self::IsLegacyIAccessiblePatternAvailable => UIA_IsLegacyIAccessiblePatternAvailablePropertyId,
            Self::IsMultipleViewPatternAvailable => UIA_IsMultipleViewPatternAvailablePropertyId,
            Self::IsObjectModelPatternAvailable => UIA_IsObjectModelPatternAvailablePropertyId,
            Self::IsOffscreen => UIA_IsOffscreenPropertyId,
            Self::IsPassword => UIA_IsPasswordPropertyId,
            Self::IsPeripheral => UIA_IsPeripheralPropertyId,
            Self::IsRangeValuePatternAvailable => UIA_IsRangeValuePatternAvailablePropertyId,
            Self::IsRequiredForForm => UIA_IsRequiredForFormPropertyId,
            Self::IsScrollItemPatternAvailable => UIA_IsScrollItemPatternAvailablePropertyId,
            Self::IsScrollPatternAvailable => UIA_IsScrollPatternAvailablePropertyId,
            Self::IsSelectionItemPatternAvailable => UIA_IsSelectionItemPatternAvailablePropertyId,
            Self::IsSelectionPattern2Available => UIA_IsSelectionPattern2AvailablePropertyId,
            Self::IsSelectionPatternAvailable => UIA_IsSelectionPatternAvailablePropertyId,
            Self::IsSpreadsheetItemPatternAvailable => UIA_IsSpreadsheetItemPatternAvailablePropertyId,
            Self::IsSpreadsheetPatternAvailable => UIA_IsSpreadsheetPatternAvailablePropertyId,
            Self::IsStylesPatternAvailable => UIA_IsStylesPatternAvailablePropertyId,
            Self::IsSynchronizedInputPatternAvailable => UIA_IsSynchronizedInputPatternAvailablePropertyId,
            Self::IsTableItemPatternAvailable => UIA_IsTableItemPatternAvailablePropertyId,
            Self::IsTablePatternAvailable => UIA_IsTablePatternAvailablePropertyId,
            Self::IsTextChildPatternAvailable => UIA_IsTextChildPatternAvailablePropertyId,
            Self::IsTextEditPatternAvailable => UIA_IsTextEditPatternAvailablePropertyId,
            Self::IsTextPattern2Available => UIA_IsTextPattern2AvailablePropertyId,
            Self::IsTextPatternAvailable => UIA_IsTextPatternAvailablePropertyId,
            Self::IsTogglePatternAvailable => UIA_IsTogglePatternAvailablePropertyId,
            Self::IsTransformPattern2Available => UIA_IsTransformPattern2AvailablePropertyId,
            Self::IsTransformPatternAvailable => UIA_IsTransformPatternAvailablePropertyId,
            Self::IsValuePatternAvailable => UIA_IsValuePatternAvailablePropertyId,
            Self::IsVirtualizedItemPatternAvailable => UIA_IsVirtualizedItemPatternAvailablePropertyId,
            Self::IsWindowPatternAvailable => UIA_IsWindowPatternAvailablePropertyId,
            Self::ItemStatus => UIA_ItemStatusPropertyId,
            Self::ItemType => UIA_ItemTypePropertyId,
            Self::LabeledBy => UIA_LabeledByPropertyId,
            Self::LandmarkType => UIA_LandmarkTypePropertyId,
            Self::LegacyIAccessibleChildId => UIA_LegacyIAccessibleChildIdPropertyId,
            Self::LegacyIAccessibleDefaultAction => UIA_LegacyIAccessibleDefaultActionPropertyId,
            Self::LegacyIAccessibleDescription => UIA_LegacyIAccessibleDescriptionPropertyId,
            Self::LegacyIAccessibleHelp => UIA_LegacyIAccessibleHelpPropertyId,
            Self::LegacyIAccessibleKeyboardShortcut => UIA_LegacyIAccessibleKeyboardShortcutPropertyId,
            Self::LegacyIAccessibleName => UIA_LegacyIAccessibleNamePropertyId,
            Self::LegacyIAccessibleRole => UIA_LegacyIAccessibleRolePropertyId,
            Self::LegacyIAccessibleSelection => UIA_LegacyIAccessibleSelectionPropertyId,
            Self::LegacyIAccessibleState => UIA_LegacyIAccessibleStatePropertyId,
            Self::LegacyIAccessibleValue => UIA_LegacyIAccessibleValuePropertyId,
            Self::Level => UIA_LevelPropertyId,
            Self::LiveSetting => UIA_LiveSettingPropertyId,
            Self::LocalizedControlType => UIA_LocalizedControlTypePropertyId,
            Self::LocalizedLandmarkType => UIA_LocalizedLandmarkTypePropertyId,
            Self::MultipleViewCurrentView => UIA_MultipleViewCurrentViewPropertyId,
            Self::MultipleViewSupportedViews => UIA_MultipleViewSupportedViewsPropertyId,
            Self::Name => UIA_NamePropertyId,
            Self::NativeWindowHandle => UIA_NativeWindowHandlePropertyId,
            Self::OptimizeForVisualContent => UIA_OptimizeForVisualContentPropertyId,
            Self::Orientation => UIA_OrientationPropertyId,
            Self::OutlineColor => UIA_OutlineColorPropertyId,
            Self::OutlineThickness => UIA_OutlineThicknessPropertyId,
            Self::PositionInSet => UIA_PositionInSetPropertyId,
            Self::ProcessId => UIA_ProcessIdPropertyId,
            Self::ProviderDescription => UIA_ProviderDescriptionPropertyId,
            Self::RangeValueIsReadOnly => UIA_RangeValueIsReadOnlyPropertyId,
            Self::RangeValueLargeChange => UIA_RangeValueLargeChangePropertyId,
            Self::RangeValueMaximum => UIA_RangeValueMaximumPropertyId,
            Self::RangeValueMinimum => UIA_RangeValueMinimumPropertyId,
            Self::RangeValueSmallChange => UIA_RangeValueSmallChangePropertyId,
            Self::RangeValueValue => UIA_RangeValueValuePropertyId,
            Self::Rotation => UIA_RotationPropertyId,
            Self::RuntimeId => UIA_RuntimeIdPropertyId,
            Self::ScrollHorizontalScrollPercent => UIA_ScrollHorizontalScrollPercentPropertyId,
            Self::ScrollHorizontalViewSize => UIA_ScrollHorizontalViewSizePropertyId,
            Self::ScrollHorizontallyScrollable => UIA_ScrollHorizontallyScrollablePropertyId,
            Self::ScrollVerticalScrollPercent => UIA_ScrollVerticalScrollPercentPropertyId,
            Self::ScrollVerticalViewSize => UIA_ScrollVerticalViewSizePropertyId,
            Self::ScrollVerticallyScrollable => UIA_ScrollVerticallyScrollablePropertyId,
            Self::Selection2CurrentSelectedItem => UIA_Selection2CurrentSelectedItemPropertyId,
            Self::Selection2FirstSelectedItem => UIA_Selection2FirstSelectedItemPropertyId,
            Self::Selection2ItemCount => UIA_Selection2ItemCountPropertyId,
            Self::Selection2LastSelectedItem => UIA_Selection2LastSelectedItemPropertyId,
            Self::SelectionCanSelectMultiple => UIA_SelectionCanSelectMultiplePropertyId,
            Self::SelectionIsSelectionRequired => UIA_SelectionIsSelectionRequiredPropertyId,
            Self::SelectionItemIsSelected => UIA_SelectionItemIsSelectedPropertyId,
            Self::SelectionItemSelectionContainer => UIA_SelectionItemSelectionContainerPropertyId,
            Self::SelectionSelection => UIA_SelectionSelectionPropertyId,
            Self::SizeOfSet => UIA_SizeOfSetPropertyId,
            Self::Size => UIA_SizePropertyId,
            Self::SpreadsheetItemAnnotationObjects => UIA_SpreadsheetItemAnnotationObjectsPropertyId,
            Self::SpreadsheetItemAnnotationTypes => UIA_SpreadsheetItemAnnotationTypesPropertyId,
            Self::SpreadsheetItemFormula => UIA_SpreadsheetItemFormulaPropertyId,
            Self::StylesExtendedProperties => UIA_StylesExtendedPropertiesPropertyId,
            Self::StylesFillColor => UIA_StylesFillColorPropertyId,
            Self::StylesFillPatternColor => UIA_StylesFillPatternColorPropertyId,
            Self::StylesFillPatternStyle => UIA_StylesFillPatternStylePropertyId,
            Self::StylesShape => UIA_StylesShapePropertyId,
            Self::StylesStyleId => UIA_StylesStyleIdPropertyId,
            Self::StylesStyleName => UIA_StylesStyleNamePropertyId,
            Self::TableColumnHeaders => UIA_TableColumnHeadersPropertyId,
            Self::TableItemColumnHeaderItems => UIA_TableItemColumnHeaderItemsPropertyId,
            Self::TableItemRowHeaderItems => UIA_TableItemRowHeaderItemsPropertyId,
            Self::TableRowHeaders => UIA_TableRowHeadersPropertyId,
            Self::TableRowOrColumnMajor => UIA_TableRowOrColumnMajorPropertyId,
            Self::ToggleToggleState => UIA_ToggleToggleStatePropertyId,
            Self::Transform2CanZoom => UIA_Transform2CanZoomPropertyId,
            Self::Transform2ZoomLevel => UIA_Transform2ZoomLevelPropertyId,
            Self::Transform2ZoomMaximum => UIA_Transform2ZoomMaximumPropertyId,
            Self::Transform2ZoomMinimum => UIA_Transform2ZoomMinimumPropertyId,
            Self::TransformCanMove => UIA_TransformCanMovePropertyId,
            Self::TransformCanResize => UIA_TransformCanResizePropertyId,
            Self::TransformCanRotate => UIA_TransformCanRotatePropertyId,
            Self::ValueIsReadOnly => UIA_ValueIsReadOnlyPropertyId,
            Self::ValueValue => UIA_ValueValuePropertyId,
            Self::VisualEffects => UIA_VisualEffectsPropertyId,
            Self::WindowCanMaximize => UIA_WindowCanMaximizePropertyId,
            Self::WindowCanMinimize => UIA_WindowCanMinimizePropertyId,
            Self::WindowIsModal => UIA_WindowIsModalPropertyId,
            Self::WindowIsTopmost => UIA_WindowIsTopmostPropertyId,
            Self::WindowWindowInteractionState => UIA_WindowWindowInteractionStatePropertyId,
            Self::WindowWindowVisualState => UIA_WindowWindowVisualStatePropertyId,
        }
    }
}