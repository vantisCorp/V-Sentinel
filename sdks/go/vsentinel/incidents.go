package vsentinel

import (
	"context"
)

// IncidentsService handles incident-related API calls.
type IncidentsService struct {
	client *Client
}

// IncidentListOptions represents options for listing incidents.
type IncidentListOptions struct {
	Query     string
	Severity  IncidentSeverity
	Status    IncidentStatus
	Phase     IncidentPhase
	Assignee  string
	TimeRange string
	SortBy    string
	SortOrder string
	Limit     int
	Offset    int
}

// toParams converts options to query parameters.
func (o *IncidentListOptions) toParams() map[string]string {
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
	if o.Phase != "" {
		params["phase"] = string(o.Phase)
	}
	if o.Assignee != "" {
		params["assignee"] = o.Assignee
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

// List retrieves a list of incidents.
func (s *IncidentsService) List(ctx context.Context, opts *IncidentListOptions) (*IncidentList, error) {
	var result IncidentList

	params := make(map[string]string)
	if opts != nil {
		params = opts.toParams()
	}

	if _, ok := params["limit"]; !ok {
		params["limit"] = "50"
	}

	err := s.client.get(ctx, "/incidents", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// Get retrieves a specific incident by ID.
func (s *IncidentsService) Get(ctx context.Context, incidentID string) (*Incident, error) {
	var result Incident

	err := s.client.get(ctx, "/incidents/"+incidentID, nil, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// CreateOptions represents options for creating an incident.
type IncidentCreateOptions struct {
	Title       string           `json:"title"`
	Description string           `json:"description"`
	Severity    IncidentSeverity `json:"severity"`
	Hosts       []string         `json:"hosts,omitempty"`
	Detections  []string         `json:"detections,omitempty"`
	Tags        []string         `json:"tags,omitempty"`
	Assignee    string           `json:"assignee,omitempty"`
}

// Create creates a new incident.
func (s *IncidentsService) Create(ctx context.Context, opts *IncidentCreateOptions) (*Incident, error) {
	var result Incident

	err := s.client.post(ctx, "/incidents", opts, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// UpdateOptions represents options for updating an incident.
type IncidentUpdateOptions struct {
	Title       string           `json:"title,omitempty"`
	Description string           `json:"description,omitempty"`
	Severity    IncidentSeverity `json:"severity,omitempty"`
	Tags        []string         `json:"tags,omitempty"`
}

// Update updates an incident.
func (s *IncidentsService) Update(ctx context.Context, incidentID string, opts *IncidentUpdateOptions) (*Incident, error) {
	var result Incident

	err := s.client.patch(ctx, "/incidents/"+incidentID, opts, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// UpdateStatusOptions represents options for updating an incident status.
type IncidentUpdateStatusOptions struct {
	Status IncidentStatus `json:"status"`
	Notes  string         `json:"notes,omitempty"`
}

// UpdateStatus updates the status of an incident.
func (s *IncidentsService) UpdateStatus(ctx context.Context, incidentID string, opts *IncidentUpdateStatusOptions) (*Incident, error) {
	var result Incident

	err := s.client.patch(ctx, "/incidents/"+incidentID+"/status", opts, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// UpdatePhaseOptions represents options for updating an incident phase.
type IncidentUpdatePhaseOptions struct {
	Phase IncidentPhase `json:"phase"`
	Notes string        `json:"notes,omitempty"`
}

// UpdatePhase updates the phase of an incident.
func (s *IncidentsService) UpdatePhase(ctx context.Context, incidentID string, opts *IncidentUpdatePhaseOptions) (*Incident, error) {
	var result Incident

	err := s.client.patch(ctx, "/incidents/"+incidentID+"/phase", opts, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// Assign assigns an incident to a user.
func (s *IncidentsService) Assign(ctx context.Context, incidentID string, assignee string) (*Incident, error) {
	var result Incident

	body := map[string]string{"assignee": assignee}

	err := s.client.patch(ctx, "/incidents/"+incidentID+"/assign", body, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// AddNote adds a note to an incident.
func (s *IncidentsService) AddNote(ctx context.Context, incidentID string, note string, visibility string) (*IncidentNote, error) {
	var result IncidentNote

	body := map[string]string{
		"content":    note,
		"visibility": visibility,
	}

	err := s.client.post(ctx, "/incidents/"+incidentID+"/notes", body, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// ListNotes lists notes for an incident.
func (s *IncidentsService) ListNotes(ctx context.Context, incidentID string, limit int) ([]IncidentNote, error) {
	var result struct {
		Notes []IncidentNote `json:"notes"`
	}

	params := map[string]string{}
	if limit > 0 {
		params["limit"] = intToString(limit)
	}

	err := s.client.get(ctx, "/incidents/"+incidentID+"/notes", params, &result)
	if err != nil {
		return nil, err
	}

	return result.Notes, nil
}

// GetTimeline retrieves the timeline for an incident.
func (s *IncidentsService) GetTimeline(ctx context.Context, incidentID string) ([]TimelineEntry, error) {
	var result struct {
		Entries []TimelineEntry `json:"entries"`
	}

	err := s.client.get(ctx, "/incidents/"+incidentID+"/timeline", nil, &result)
	if err != nil {
		return nil, err
	}

	return result.Entries, nil
}

// AddHosts adds hosts to an incident.
func (s *IncidentsService) AddHosts(ctx context.Context, incidentID string, hostIDs []string) (*Incident, error) {
	var result Incident

	body := map[string][]string{"host_ids": hostIDs}

	err := s.client.post(ctx, "/incidents/"+incidentID+"/hosts", body, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// RemoveHosts removes hosts from an incident.
func (s *IncidentsService) RemoveHosts(ctx context.Context, incidentID string, hostIDs []string) (*Incident, error) {
	var result Incident

	body := map[string][]string{"host_ids": hostIDs}

	err := s.client.delete(ctx, "/incidents/"+incidentID+"/hosts", body, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// AddDetections adds detections to an incident.
func (s *IncidentsService) AddDetections(ctx context.Context, incidentID string, detectionIDs []string) (*Incident, error) {
	var result Incident

	body := map[string][]string{"detection_ids": detectionIDs}

	err := s.client.post(ctx, "/incidents/"+incidentID+"/detections", body, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// RemoveDetections removes detections from an incident.
func (s *IncidentsService) RemoveDetections(ctx context.Context, incidentID string, detectionIDs []string) (*Incident, error) {
	var result Incident

	body := map[string][]string{"detection_ids": detectionIDs}

	err := s.client.delete(ctx, "/incidents/"+incidentID+"/detections", body, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// CloseOptions represents options for closing an incident.
type IncidentCloseOptions struct {
	Resolution      string `json:"resolution"`
	LessonsLearned  string `json:"lessons_learned,omitempty"`
}

// Close closes an incident.
func (s *IncidentsService) Close(ctx context.Context, incidentID string, opts *IncidentCloseOptions) (*Incident, error) {
	var result Incident

	err := s.client.post(ctx, "/incidents/"+incidentID+"/close", opts, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// Search searches for incidents matching a query.
func (s *IncidentsService) Search(ctx context.Context, query string, limit int) (*IncidentList, error) {
	var result IncidentList

	params := map[string]string{
		"query": query,
	}
	if limit > 0 {
		params["limit"] = intToString(limit)
	} else {
		params["limit"] = "50"
	}

	err := s.client.get(ctx, "/incidents/search", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}