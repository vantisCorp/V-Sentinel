package vsentinel

import (
	"context"
)

// DetectionsService handles detection-related API calls.
type DetectionsService struct {
	client *Client
}

// DetectionListOptions represents options for listing detections.
type DetectionListOptions struct {
	Query     string
	Severity  DetectionSeverity
	Status    DetectionStatus
	HostID    string
	TimeRange string
	SortBy    string
	SortOrder string
	Limit     int
	Offset    int
}

// toParams converts options to query parameters.
func (o *DetectionListOptions) toParams() map[string]string {
	params := make(map[string]string)

	if o.Query != "" {
		params["query"] = o.Query
	}
	if o.Severity != "" {
		params["severity"] = string(o.Severity)
	}
	if o.Status != "" {
		params["status"] = string(o.Status)
	}
	if o.HostID != "" {
		params["host_id"] = o.HostID
	}
	if o.TimeRange != "" {
		params["time_range"] = o.TimeRange
	}
	if o.SortBy != "" {
		params["sort_by"] = o.SortBy
	}
	if o.SortOrder != "" {
		params["sort_order"] = o.SortOrder
	}
	if o.Limit > 0 {
		params["limit"] = intToString(o.Limit)
	}
	if o.Offset > 0 {
		params["offset"] = intToString(o.Offset)
	}

	return params
}

// List retrieves a list of detections.
func (s *DetectionsService) List(ctx context.Context, opts *DetectionListOptions) (*DetectionList, error) {
	var result DetectionList

	params := make(map[string]string)
	if opts != nil {
		params = opts.toParams()
	}

	if _, ok := params["limit"]; !ok {
		params["limit"] = "50"
	}

	err := s.client.get(ctx, "/detections", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// Get retrieves a specific detection by ID.
func (s *DetectionsService) Get(ctx context.Context, detectionID string) (*Detection, error) {
	var result Detection

	err := s.client.get(ctx, "/detections/"+detectionID, nil, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// Search searches for detections matching a query.
func (s *DetectionsService) Search(ctx context.Context, query string, limit int) (*DetectionList, error) {
	var result DetectionList

	params := map[string]string{
		"query": query,
	}
	if limit > 0 {
		params["limit"] = intToString(limit)
	} else {
		params["limit"] = "50"
	}

	err := s.client.get(ctx, "/detections/search", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// UpdateStatusOptions represents options for updating a detection status.
type UpdateStatusOptions struct {
	Status DetectionStatus `json:"status"`
	Notes  string          `json:"notes,omitempty"`
}

// UpdateStatus updates the status of a detection.
func (s *DetectionsService) UpdateStatus(ctx context.Context, detectionID string, opts *UpdateStatusOptions) (*Detection, error) {
	var result Detection

	err := s.client.patch(ctx, "/detections/"+detectionID+"/status", opts, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// Assign assigns a detection to a user.
func (s *DetectionsService) Assign(ctx context.Context, detectionID string, assignee string) (*Detection, error) {
	var result Detection

	body := map[string]string{"assignee": assignee}

	err := s.client.patch(ctx, "/detections/"+detectionID+"/assign", body, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// AddNote adds a note to a detection.
func (s *DetectionsService) AddNote(ctx context.Context, detectionID string, note string) (*DetectionNote, error) {
	var result DetectionNote

	body := map[string]string{"content": note}

	err := s.client.post(ctx, "/detections/"+detectionID+"/notes", body, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// ListNotes lists notes for a detection.
func (s *DetectionsService) ListNotes(ctx context.Context, detectionID string, limit int) ([]DetectionNote, error) {
	var result struct {
		Notes []DetectionNote `json:"notes"`
	}

	params := map[string]string{}
	if limit > 0 {
		params["limit"] = intToString(limit)
	}

	err := s.client.get(ctx, "/detections/"+detectionID+"/notes", params, &result)
	if err != nil {
		return nil, err
	}

	return result.Notes, nil
}