package vsentinel

import (
	"context"
)

// HostsService handles host-related API calls.
type HostsService struct {
	client *Client
}

// HostListOptions represents options for listing hosts.
type HostListOptions struct {
	Query      string
	Platform   HostPlatform
	State      HostState
	GroupID    string
	IsIsolated *bool
	Limit      int
	Offset     int
}

// toParams converts options to query parameters.
func (o *HostListOptions) toParams() map[string]string {
	params := make(map[string]string)

	if o.Query != "" {
		params["query"] = o.Query
	}
	if o.Platform != "" {
		params["platform"] = string(o.Platform)
	}
	if o.State != "" {
		params["state"] = string(o.State)
	}
	if o.GroupID != "" {
		params["group_id"] = o.GroupID
	}
	if o.IsIsolated != nil {
		if *o.IsIsolated {
			params["is_isolated"] = "true"
		} else {
			params["is_isolated"] = "false"
		}
	}
	if o.Limit > 0 {
		params["limit"] = intToString(o.Limit)
	}
	if o.Offset > 0 {
		params["offset"] = intToString(o.Offset)
	}

	return params
}

// List retrieves a list of hosts.
func (s *HostsService) List(ctx context.Context, opts *HostListOptions) (*HostList, error) {
	var result HostList

	params := make(map[string]string)
	if opts != nil {
		params = opts.toParams()
	}

	if _, ok := params["limit"]; !ok {
		params["limit"] = "50"
	}

	err := s.client.get(ctx, "/hosts", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// Get retrieves a specific host by ID.
func (s *HostsService) Get(ctx context.Context, hostID string) (*Host, error) {
	var result Host

	err := s.client.get(ctx, "/hosts/"+hostID, nil, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// Search searches for hosts matching a query.
func (s *HostsService) Search(ctx context.Context, query string, limit int) (*HostList, error) {
	var result HostList

	params := map[string]string{
		"query": query,
	}
	if limit > 0 {
		params["limit"] = intToString(limit)
	} else {
		params["limit"] = "50"
	}

	err := s.client.get(ctx, "/hosts/search", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// Isolate isolates a host from the network.
func (s *HostsService) Isolate(ctx context.Context, hostID string, reason string) (*Host, error) {
	var result Host

	body := map[string]string{"reason": reason}

	err := s.client.post(ctx, "/hosts/"+hostID+"/isolate", body, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// Unisolate removes network isolation from a host.
func (s *HostsService) Unisolate(ctx context.Context, hostID string) (*Host, error) {
	var result Host

	err := s.client.post(ctx, "/hosts/"+hostID+"/unisolate", nil, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// GetDetections retrieves detections for a specific host.
func (s *HostsService) GetDetections(ctx context.Context, hostID string, limit int) (*DetectionList, error) {
	var result DetectionList

	params := map[string]string{}
	if limit > 0 {
		params["limit"] = intToString(limit)
	} else {
		params["limit"] = "20"
	}

	err := s.client.get(ctx, "/hosts/"+hostID+"/detections", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// AddTag adds a tag to a host.
func (s *HostsService) AddTag(ctx context.Context, hostID string, tag string) (*Host, error) {
	var result Host

	body := map[string]string{"tag": tag}

	err := s.client.post(ctx, "/hosts/"+hostID+"/tags", body, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// RemoveTag removes a tag from a host.
func (s *HostsService) RemoveTag(ctx context.Context, hostID string, tag string) error {
	body := map[string]string{"tag": tag}

	return s.client.delete(ctx, "/hosts/"+hostID+"/tags", body, nil)
}

// GetIncidents retrieves incidents for a specific host.
func (s *HostsService) GetIncidents(ctx context.Context, hostID string, limit int) (*IncidentList, error) {
	var result IncidentList

	params := map[string]string{}
	if limit > 0 {
		params["limit"] = intToString(limit)
	} else {
		params["limit"] = "20"
	}

	err := s.client.get(ctx, "/hosts/"+hostID+"/incidents", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}